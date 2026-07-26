use crate::search::{search::Search, search_options::SearchOptions};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::{CastlingMode, Chess, Position};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

const SEARCH_STACK_SIZE: usize = 8 * 1024 * 1024; // 8 MB

pub(crate) struct Uci {
    pub(crate) name: String,
    pub(crate) author: String,
    stop: Arc<AtomicBool>,
    search_options: SearchOptions,
    search_handle: Option<JoinHandle<()>>,
}

impl Default for Uci {
    fn default() -> Self {
        Uci {
            name: String::from("Pluto"),
            author: String::from("Ludovic Debever"),
            stop: Arc::new(AtomicBool::new(false)),
            search_options: SearchOptions::default(),
            search_handle: None,
        }
    }
}

impl Uci {
    pub(crate) fn run(&mut self) {
        self.command_uci();
        self.run_loop();
    }

    fn run_loop(&mut self) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            self.parse_command(input);
        }
    }

    fn parse_command(&mut self, input: &str) {
        let mut queue: VecDeque<&str> = input.split(" ").collect();
        let command = queue.pop_front().unwrap_or("");

        match command {
            "uci" => self.command_uci(),
            "isready" => self.command_isready(),
            "go" => self.command_go(&mut queue),
            "position" => self.command_position(&mut queue),
            "quit" => self.command_quit(),
            "setoption" => self.command_option(&mut queue),
            "bench" => self.command_bench(),
            "stop" => self.command_stop(),
            "ucinewgame" => {} // TODO: implement ucinewgame command
            "debug" => self.command_debug(&mut queue),
            _ => {}
        }
    }

    fn command_debug(&mut self, _queue: &mut VecDeque<&str>) {
        println!("{:?}", self.search_options.position.board());
    }

    fn command_position(&mut self, queue: &mut VecDeque<&str>) {
        let mut position = Chess::default();

        while let Some(arg) = queue.pop_front() {
            match arg {
                "startpos" => position = Chess::default(),
                "fen" => self.handle_position_fen(&mut position, queue),
                "moves" => self.handle_position_moves(&mut position, queue),
                _ => {}
            }
        }

        self.search_options.position = position;
    }

    fn handle_position_fen(&mut self, position: &mut Chess, queue: &mut VecDeque<&str>) {
        let fen_parts: Vec<String> = queue.drain(..6).map(|s| s.to_string()).collect();
        let fen: Fen = fen_parts
            .join(" ")
            .parse()
            .unwrap_or_else(|_| Fen::default());

        *position = fen
            .into_position(CastlingMode::Standard)
            .unwrap_or_else(|_| Chess::default());
    }

    fn handle_position_moves(&mut self, position: &mut Chess, queue: &mut VecDeque<&str>) {
        for move_str in queue.drain(..) {
            let uci_move = move_str.parse().unwrap_or(UciMove::Null);

            if let Ok(mv) = uci_move.to_move(position) {
                *position = position
                    .clone()
                    .play(mv)
                    .unwrap_or_else(|_| position.clone());
            }
        }
    }

    pub(crate) fn command_bench(&mut self) {
        self.stop.store(false, Ordering::Relaxed);

        let search_options = SearchOptions::default().depth(6);
        let stop = Arc::clone(&self.stop);
        let handle = std::thread::Builder::new()
            .stack_size(SEARCH_STACK_SIZE)
            .spawn(move || {
                let mut search = Search::from(&search_options, stop);
                let result = search.run();

                println!("{} nodes {} nps", result.nodes, result.nps);
            })
            .unwrap();

        // Store the handle of the new search thread
        self.search_handle = Some(handle);

        if let Some(handle) = self.search_handle.take() {
            handle.join().unwrap();
        }
    }

    fn command_option(&mut self, _queue: &mut VecDeque<&str>) {
        let name = _queue.pop_front().unwrap_or("");

        // TODO: may need to print "ignoring"
        if name != "name" {
            return;
        }

        let value = _queue.pop_front().unwrap_or("");

        match value {
            "Hash" => {}
            "Threads" => {}
            _ => {}
        }
    }

    fn command_stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);

        if let Some(handle) = self.search_handle.take() {
            handle.join().unwrap();
        }
    }

    fn command_quit(&mut self) {
        // TODO: Maybe we dont want to wait for search to end before quitting, but for now we will wait
        self.command_stop();

        std::process::exit(0);
    }

    fn command_uci(&self) {
        println!("id name {}", self.name);
        println!("id author {}", self.author);
        println!("option name Hash type spin default 1 min 1 max 1");
        println!("option name Threads type spin default 1 min 1 max 1");
        println!("uciok");
    }

    fn command_isready(&self) {
        println!("readyok");
    }

    fn command_go(&mut self, queue: &mut VecDeque<&str>) {
        self.search_options.reset();

        #[cfg_attr(any(), rustfmt::skip)]
        while let Some(arg) = queue.pop_front() {
            match arg {
                "depth" => self.search_options.depth = queue.pop_front().and_then(|s| s.parse::<u32>().ok()),
                "nodes" => self.search_options.nodes = queue.pop_front().and_then(|s| s.parse::<u64>().ok()),
                "movetime" => self.search_options.move_time = queue.pop_front().and_then(|s| s.parse::<u64>().ok()),
                "wtime" => self.search_options.wtime = queue.pop_front().and_then(|s| s.parse::<u64>().ok()),
                "btime" => self.search_options.btime = queue.pop_front().and_then(|s| s.parse::<u64>().ok()),
                _ => {}
            }
        }

        // Wait for any existing search thread to finish before starting a new one
        if let Some(handle) = self.search_handle.take() {
            handle.join().unwrap();
        }

        // Reset the stop flag before starting a new search
        self.stop.store(false, Ordering::Relaxed);

        let stop = Arc::clone(&self.stop);
        let search_options = self.search_options.clone();
        let handle = std::thread::Builder::new()
            .stack_size(SEARCH_STACK_SIZE)
            .spawn(move || {
                let mut search = Search::from(&search_options, stop);
                let result = search.run();

                println!("bestmove {}", result.best_move);
            })
            .unwrap();

        // Store the handle of the new search thread
        self.search_handle = Some(handle);
    }
}
