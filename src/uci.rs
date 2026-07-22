use crate::search::{search::Search, search_options::SearchOptions};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

const SEARCH_STACK_SIZE: usize = 8 * 1024 * 1024; // 8 MB

pub(crate) struct Uci {
    pub(crate) name: String,
    pub(crate) author: String,
    stop: Arc<AtomicBool>,
    search_handle: Option<JoinHandle<()>>,
}

impl Default for Uci {
    fn default() -> Self {
        Uci {
            name: String::from("Pluto"),
            author: String::from("Ludovic Debever"),
            stop: Arc::new(AtomicBool::new(false)),
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
            "quit" => self.command_quit(),
            "setoption" => self.command_option(&mut queue),
            "bench" => self.command_bench(),
            "stop" => self.command_stop(),
            _ => {}
        }
    }

    pub(crate) fn command_bench(&mut self) {
        self.stop.store(false, Ordering::Relaxed);

        let search_options = SearchOptions::default().depth(5);
        let stop = Arc::clone(&self.stop);
        let handle = std::thread::Builder::new()
            .stack_size(SEARCH_STACK_SIZE)
            .spawn(move || {
                let result = Search::run(&search_options, stop);

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
        let mut search_options = SearchOptions::default();

        #[cfg_attr(any(), rustfmt::skip)]
        while let Some(arg) = queue.pop_front() {
            match arg {
                "depth" => search_options.depth = queue.pop_front().and_then(|s| s.parse::<u32>().ok()),
                "nodes" => search_options.nodes = queue.pop_front().and_then(|s| s.parse::<u64>().ok()),
                "movetime" => search_options.movetime = queue.pop_front().and_then(|s| s.parse::<u64>().ok()),
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
        let handle = std::thread::Builder::new()
            .stack_size(SEARCH_STACK_SIZE)
            .spawn(move || {
                let result = Search::run(&search_options, stop);

                println!("bestmove {}", result.bestmove);
            })
            .unwrap();

        // Store the handle of the new search thread
        self.search_handle = Some(handle);
    }
}
