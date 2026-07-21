use crate::search::{search_options::SearchOptions, search_result::SearchResult};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{thread, time};

pub(crate) struct Search;

impl Search {
    pub(crate) fn run(options: &SearchOptions, _stop: Arc<AtomicBool>) -> SearchResult {
        println!(
            "Running search with options: depth={:?}, nodes={:?}, movetime={:?}",
            options.depth, options.nodes, options.movetime
        );

        // PLACEHOLDER: Simulate a search by sleeping for a short duration
        let time = time::Duration::from_millis(100);

        for _ in 0..100 {
            if _stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            thread::sleep(time);
        }

        SearchResult {
            bestmove: String::from("e2e4"),
        }
    }
}
