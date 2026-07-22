use shakmaty::Chess;

pub(crate) struct SearchOptions {
    pub(crate) depth: Option<u32>,
    pub(crate) nodes: Option<u64>,
    pub(crate) movetime: Option<u64>,
    pub(crate) position: Chess,
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            depth: None,
            nodes: None,
            movetime: None,
            position: Chess::default(),
        }
    }
}

impl SearchOptions {
    pub(crate) fn depth(mut self, depth: u32) -> Self {
        self.depth = Some(depth);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn nodes(mut self, nodes: u64) -> Self {
        self.nodes = Some(nodes);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn movetime(mut self, movetime: u64) -> Self {
        self.movetime = Some(movetime);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn position(mut self, position: Chess) -> Self {
        self.position = position;
        self
    }
}
