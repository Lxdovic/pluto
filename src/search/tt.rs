use shakmaty::{Move, zobrist::Zobrist64};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TTBound {
    Exact,
    Beta,
    Alpha,
}

#[derive(Debug, Clone)]
pub(crate) struct TTEntry {
    pub(crate) sig: u16,
    pub(crate) depth: u8,
    pub(crate) score: i16,
    pub(crate) bound: TTBound,
    // pub(crate) best_move: [u8; 2],
    pub(crate) best_move: Option<Move>,
}

impl Default for TTEntry {
    fn default() -> Self {
        TTEntry {
            sig: 0,
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
    pub(crate) fn new(size: u16) -> Self {
        let length = size as usize * 1024 * 1024 / std::mem::size_of::<TTEntry>();

        Self {
            table: vec![TTEntry::default(); length],
            length,
        }
    }

    pub(crate) fn probe(&self, key: Zobrist64) -> Option<&TTEntry> {
        let index = key.0 as usize % self.length;

        let entry = &self.table[index];

        if entry.sig != key.0 as u16 {
            return None;
        }

        Some(&entry)
    }

    pub(crate) fn store(
        &mut self,
        key: Zobrist64,
        depth: u8,
        score: i16,
        bound: TTBound,
        best_move: Option<Move>,
    ) {
        let index = key.0 as usize % self.length;
        let entry = TTEntry {
            sig: key.0 as u16,
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
