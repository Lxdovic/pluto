use crate::search::{search::Search, search_options::SearchOptions};
use std::collections::VecDeque;

pub(crate) struct Uci {
    pub(crate) name: String,
    pub(crate) author: String,
}

impl Default for Uci {
    fn default() -> Self {
        Uci {
            name: String::from("Pluto"),
            author: String::from("Ludovic Debever"),
        }
    }
}

impl Uci {
    pub(crate) fn run(&self) {
        self.command_uci();
        self.run_loop();
    }

    fn run_loop(&self) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            self.parse_command(input);
        }
    }

    fn parse_command(&self, input: &str) {
        let mut queue: VecDeque<&str> = input.split(" ").collect();
        let command = queue.pop_front().unwrap_or("");

        match command {
            "uci" => self.command_uci(),
            "isready" => self.command_isready(),
            "go" => self.command_go(&mut queue),
            "quit" => std::process::exit(0),
            _ => {}
        }
    }

    fn command_uci(&self) {
        println!("id name {}", self.name);
        println!("id author {}", self.author);
        println!("uciok");
    }

    fn command_isready(&self) {
        println!("readyok");
    }

    fn command_go(&self, queue: &mut VecDeque<&str>) {
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

        let result = Search.run(&search_options);

        println!("bestmove {}", result.bestmove);
    }
}
