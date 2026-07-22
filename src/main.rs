use std::env;

pub mod search;
pub mod uci;

fn main() {
    for argument in env::args() {
        if argument == "bench" {
            uci::Uci::default().command_bench();

            return;
        }
    }

    uci::Uci::default().run();
}
