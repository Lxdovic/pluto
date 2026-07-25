use shakmaty::{Chess, Color, Position};

#[derive(Debug, Clone)]
pub(crate) struct SearchOptions {
    pub(crate) depth: Option<u32>,
    pub(crate) nodes: Option<u64>,
    pub(crate) move_time: Option<u64>,
    pub(crate) position: Chess,
    pub(crate) wtime: Option<u64>,
    pub(crate) btime: Option<u64>,
}

#[derive(Debug, Clone)]
pub(crate) struct BuiltSearchOptions {
    pub(crate) depth: Option<u32>,
    pub(crate) nodes: Option<u64>,
    pub(crate) time: Option<u64>,
    pub(crate) position: Chess,
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            depth: None,
            nodes: None,
            wtime: None,
            btime: None,
            move_time: None,
            position: Chess::default(),
        }
    }
}

impl SearchOptions {
    pub fn build(&self) -> BuiltSearchOptions {
        let turn_time = match self.position.turn() {
            Color::White => self.wtime,
            Color::Black => self.btime,
        };

        BuiltSearchOptions {
            depth: self.depth,
            nodes: self.nodes,
            position: self.position.clone(),
            time: self.move_time.or(turn_time),
        }
    }

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
    pub(crate) fn move_time(mut self, move_time: u64) -> Self {
        self.move_time = Some(move_time);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn position(mut self, position: Chess) -> Self {
        self.position = position;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn wtime(mut self, wtime: u64) -> Self {
        self.wtime = Some(wtime);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn btime(mut self, btime: u64) -> Self {
        self.btime = Some(btime);
        self
    }
}
