use shakmaty::{Move, zobrist::Zobrist64};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TTBound {
    Exact,
    Beta,
    Alpha,
}

#[derive(Debug, Clone)]
pub(crate) struct TTEntry {
    pub(crate) key: Zobrist64,
    pub(crate) depth: u32,
    pub(crate) score: i32,
    pub(crate) bound: TTBound,
    // pub(crate) best_move: [u8; 2],
    pub(crate) best_move: Option<Move>,
}

impl Default for TTEntry {
    fn default() -> Self {
        TTEntry {
            key: Zobrist64(0),
            depth: 0,
            score: 0,
            bound: TTBound::Exact,
            best_move: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TranspositionTable {
    pub table: Vec<TTEntry>,
    length: usize,
}

impl TranspositionTable {
    pub(crate) fn new(length: usize) -> Self {
        Self {
            table: vec![TTEntry::default(); length],
            length,
        }
    }

    pub(crate) fn probe(&self, key: Zobrist64) -> &TTEntry {
        let index = key.0 as usize % self.length;

        &self.table[index]
    }

    pub(crate) fn store(
        &mut self,
        key: Zobrist64,
        depth: u32,
        score: i32,
        bound: TTBound,
        best_move: Option<Move>,
    ) {
        let index = key.0 as usize % self.table.len();
        let entry = TTEntry {
            key,
            depth,
            score,
            bound,
            best_move,
        };

        self.table[index] = entry;
    }

    pub(crate) fn clear(&mut self) {
        self.table = vec![TTEntry::default(); self.length];
    }
}
