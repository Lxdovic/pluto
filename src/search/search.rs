use crate::search::{search_options::SearchOptions, search_result::SearchResult};

pub(crate) struct Search;

impl Search {
    pub(crate) fn run(&self, options: &SearchOptions) -> SearchResult {
        println!(
            "Running search with options: depth={:?}, nodes={:?}, movetime={:?}",
            options.depth, options.nodes, options.movetime
        );

        SearchResult {
            bestmove: String::from("e2e4"),
        }
    }
}
