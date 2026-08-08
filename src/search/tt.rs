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
    pub(crate) depth: u8,
    pub(crate) score: i16,
    pub(crate) bound: TTBound,
    pub(crate) best_move: Option<Move>,
    pub(crate) generation: u8,
}

impl Default for TTEntry {
    fn default() -> Self {
        TTEntry {
            key: Zobrist64(0),
            depth: 0,
            score: 0,
            generation: 0,
            bound: TTBound::Exact,
            best_move: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TranspositionTable {
    pub table: Vec<TTEntry>,
    length: usize,
    generation: u8,
}

impl TranspositionTable {
    pub(crate) fn new(size: u16) -> Self {
        let length = size as usize * 1024 * 1024 / std::mem::size_of::<TTEntry>();

        Self {
            table: vec![TTEntry::default(); length],
            length,
            generation: 0,
        }
    }

    pub(crate) fn generation(&self) -> u8 {
        self.generation
    }

    pub(crate) fn bump_generation(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    pub(crate) fn probe(&self, key: Zobrist64) -> Option<&TTEntry> {
        let index = key.0 as usize % self.length;
        let entry = &self.table[index];

        if entry.key != key {
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
            key,
            depth,
            score,
            bound,
            best_move,
            generation: self.generation,
        };

        self.table[index] = entry;
    }

    pub(crate) fn clear(&mut self) {
        self.table = vec![TTEntry::default(); self.length];
        self.generation = 0;
    }
}
