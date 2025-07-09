use crate::types::{Square, Color};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const ALL: Bitboard = Bitboard(0xFFFFFFFFFFFFFFFF);

    // Rank masks
    pub const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
    pub const RANK_2: Bitboard = Bitboard(0x000000000000FF00);
    pub const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
    pub const RANK_4: Bitboard = Bitboard(0x00000000FF000000);
    pub const RANK_5: Bitboard = Bitboard(0x000000FF00000000);
    pub const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
    pub const RANK_7: Bitboard = Bitboard(0x00FF000000000000);
    pub const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

    // File masks
    pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
    pub const FILE_B: Bitboard = Bitboard(0x0202020202020202);
    pub const FILE_C: Bitboard = Bitboard(0x0404040404040404);
    pub const FILE_D: Bitboard = Bitboard(0x0808080808080808);
    pub const FILE_E: Bitboard = Bitboard(0x1010101010101010);
    pub const FILE_F: Bitboard = Bitboard(0x2020202020202020);
    pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
    pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);

    #[inline]
    pub fn new(value: u64) -> Self {
        Bitboard(value)
    }

    #[inline]
    pub fn from_square(square: Square) -> Self {
        Bitboard(1u64 << (square as u8))
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_set(self, square: Square) -> bool {
        (self.0 & (1u64 << (square as u8))) != 0
    }

    #[inline]
    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << (square as u8);
    }

    #[inline]
    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << (square as u8));
    }

    #[inline]
    pub fn toggle(&mut self, square: Square) {
        self.0 ^= 1u64 << (square as u8);
    }

    #[inline]
    pub fn pop_count(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }

    #[inline]
    pub fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    #[inline]
    pub fn lsb(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square::from_index(self.trailing_zeros() as u8))
        }
    }

    #[inline]
    pub fn msb(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square::from_index(63 - self.leading_zeros() as u8))
        }
    }

    #[inline]
    pub fn pop_lsb(&mut self) -> Option<Square> {
        let square = self.lsb()?;
        self.0 &= self.0 - 1; // Clear the least significant bit
        Some(square)
    }

    #[inline]
    pub fn shift_north(self) -> Self {
        Bitboard(self.0 << 8)
    }

    #[inline]
    pub fn shift_south(self) -> Self {
        Bitboard(self.0 >> 8)
    }

    #[inline]
    pub fn shift_east(self) -> Self {
        Bitboard((self.0 << 1) & !Self::FILE_A.0)
    }

    #[inline]
    pub fn shift_west(self) -> Self {
        Bitboard((self.0 >> 1) & !Self::FILE_H.0)
    }

    #[inline]
    pub fn shift_northeast(self) -> Self {
        Bitboard((self.0 << 9) & !Self::FILE_A.0)
    }

    #[inline]
    pub fn shift_northwest(self) -> Self {
        Bitboard((self.0 << 7) & !Self::FILE_H.0)
    }

    #[inline]
    pub fn shift_southeast(self) -> Self {
        Bitboard((self.0 >> 7) & !Self::FILE_A.0)
    }

    #[inline]
    pub fn shift_southwest(self) -> Self {
        Bitboard((self.0 >> 9) & !Self::FILE_H.0)
    }

    #[inline]
    pub fn rank(rank: u8) -> Self {
        match rank {
            0 => Self::RANK_1,
            1 => Self::RANK_2,
            2 => Self::RANK_3,
            3 => Self::RANK_4,
            4 => Self::RANK_5,
            5 => Self::RANK_6,
            6 => Self::RANK_7,
            7 => Self::RANK_8,
            _ => Self::EMPTY,
        }
    }

    #[inline]
    pub fn file(file: u8) -> Self {
        match file {
            0 => Self::FILE_A,
            1 => Self::FILE_B,
            2 => Self::FILE_C,
            3 => Self::FILE_D,
            4 => Self::FILE_E,
            5 => Self::FILE_F,
            6 => Self::FILE_G,
            7 => Self::FILE_H,
            _ => Self::EMPTY,
        }
    }

    pub fn iter(self) -> BitboardIterator {
        BitboardIterator { bitboard: self }
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

pub struct BitboardIterator {
    bitboard: Bitboard,
}

impl Iterator for BitboardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.bitboard.pop_lsb()
    }
}

// Precomputed attack tables
pub struct AttackTables {
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
    pub pawn_attacks: [[Bitboard; 64]; 2], // [color][square]
}

impl AttackTables {
    pub fn new() -> Self {
        let mut tables = AttackTables {
            knight_attacks: [Bitboard::EMPTY; 64],
            king_attacks: [Bitboard::EMPTY; 64],
            pawn_attacks: [[Bitboard::EMPTY; 64]; 2],
        };

        // Generate knight attacks
        for square in 0..64 {
            let sq = Square::from_index(square);
            let file = sq.file() as i8;
            let rank = sq.rank() as i8;

            let mut attacks = Bitboard::EMPTY;
            
            let knight_moves = [
                (-2, -1), (-2, 1), (-1, -2), (-1, 2),
                (1, -2), (1, 2), (2, -1), (2, 1)
            ];

            for (df, dr) in knight_moves {
                let new_file = file + df;
                let new_rank = rank + dr;
                
                if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                    if let Some(target) = Square::new(new_file as u8, new_rank as u8) {
                        attacks |= Bitboard::from_square(target);
                    }
                }
            }
            
            tables.knight_attacks[square as usize] = attacks;
        }

        // Generate king attacks
        for square in 0..64 {
            let sq = Square::from_index(square);
            let file = sq.file() as i8;
            let rank = sq.rank() as i8;

            let mut attacks = Bitboard::EMPTY;
            
            let king_moves = [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),  (1, 0),  (1, 1)
            ];

            for (df, dr) in king_moves {
                let new_file = file + df;
                let new_rank = rank + dr;
                
                if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                    if let Some(target) = Square::new(new_file as u8, new_rank as u8) {
                        attacks |= Bitboard::from_square(target);
                    }
                }
            }
            
            tables.king_attacks[square as usize] = attacks;
        }

        // Generate pawn attacks
        for square in 0..64 {
            let sq = Square::from_index(square);
            let file = sq.file() as i8;
            let rank = sq.rank() as i8;

            // White pawn attacks (moving up)
            let mut white_attacks = Bitboard::EMPTY;
            if rank < 7 {
                if file > 0 {
                    if let Some(target) = Square::new((file - 1) as u8, (rank + 1) as u8) {
                        white_attacks |= Bitboard::from_square(target);
                    }
                }
                if file < 7 {
                    if let Some(target) = Square::new((file + 1) as u8, (rank + 1) as u8) {
                        white_attacks |= Bitboard::from_square(target);
                    }
                }
            }
            tables.pawn_attacks[Color::White as usize][square as usize] = white_attacks;

            // Black pawn attacks (moving down)
            let mut black_attacks = Bitboard::EMPTY;
            if rank > 0 {
                if file > 0 {
                    if let Some(target) = Square::new((file - 1) as u8, (rank - 1) as u8) {
                        black_attacks |= Bitboard::from_square(target);
                    }
                }
                if file < 7 {
                    if let Some(target) = Square::new((file + 1) as u8, (rank - 1) as u8) {
                        black_attacks |= Bitboard::from_square(target);
                    }
                }
            }
            tables.pawn_attacks[Color::Black as usize][square as usize] = black_attacks;
        }

        tables
    }
}

lazy_static::lazy_static! {
    pub static ref ATTACK_TABLES: AttackTables = AttackTables::new();
}