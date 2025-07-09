use crate::chess::{Bitboard, Square};

// Magic numbers for rooks (precomputed)
const ROOK_MAGICS: [u64; 64] = [
    0x8a80104000800020, 0x140002000100040,  0x2801880a0017001,  0x100081001000420,
    0x200020010080420,  0x3001c0002010008,  0x8480008002000100, 0x2080088004402900,
    0x800098204000,     0x2024401000200040, 0x100802000801000,  0x120800800801000,
    0x208808088000400,  0x2802200800400,    0x2200800100020080, 0x801000060821100,
    0x80044006422000,   0x100808020004000,  0x12108a0010204200, 0x140848010000802,
    0x481828014002800,  0x8094004002004100, 0x4010040010010802, 0x20008806104,
    0x100400080208000,  0x2040002120081000, 0x21200680100081,   0x20100080080080,
    0x2000a00200410,    0x20080800400,      0x80088400100102,   0x80004600042881,
    0x4040008040800020, 0x440003000200801,  0x4200011004500,    0x188020010100100,
    0x14800401802800,   0x2080040080800200, 0x124080204001001,  0x200046502000484,
    0x480400080088020,  0x1000422010034000, 0x30200100110040,   0x100021010009,
    0x2002080100110004, 0x202008004008002,  0x20020004010100,   0x2048440040820001,
    0x101002200408200,  0x40802000401080,   0x4008142004410100, 0x2060820c0120200,
    0x1001004080100,    0x20c020080040080,  0x2935610830022400, 0x44440041009200,
    0x280001040802101,  0x2100190040002085, 0x80c0084100102001, 0x4024081001000421,
    0x20030a0244872,    0x12001008414402,   0x2006104900a0804,  0x1004081002402,
];

// Magic numbers for bishops (precomputed)
const BISHOP_MAGICS: [u64; 64] = [
    0x40040844404084,   0x2004208a004208,   0x10190041080202,   0x108060845042010,
    0x581104180800210,  0x2112080446200010, 0x1080820820060210, 0x3c0808410220200,
    0x4050404440404,    0x21001420088,      0x24d0080801082102, 0x1020a0a020400,
    0x40308200402,      0x4011002100800,    0x401484104104005,  0x801010402020200,
    0x400210c3880100,   0x404022024108200,  0x810018200204102,  0x4002801a02003,
    0x85040820080400,   0x810102c808880400, 0x84040420020,      0x8000094001100a00,
    0x100442060002009,  0x4480041000041c80, 0x8000a05004100c00, 0x40100400a040,
    0x80410001001000,   0x208042000808200,  0x80402401800080,   0x802c40020004080,
    0x82048020004004,   0x202a3240088804,   0x88500208a0208,    0x1822080084404,
    0x800404400480,     0x200040020020100,  0x80220010220080,   0x8000400a204200,
    0x8005050028048,    0x1080022080020,    0x8004405410020,    0x222004002000800,
    0x4000080800020,    0x4001082080401100, 0x8008020800040800, 0x2000010020004004,
    0x4004004020080102, 0x1002002008002080, 0x204800040004101,  0x20221040000104,
    0x44040040202101,   0x820c00c020008,    0x1002002010008,    0x40010080005004,
    0x40020040088,      0x1002001008,       0x4008402000008,    0x8004014000,
    0x200a800040800020, 0x1000200040400020, 0x8000400020020,    0x40020040020040,
];

// Relevant occupancy bit counts for rooks
const ROOK_RELEVANT_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

// Relevant occupancy bit counts for bishops
const BISHOP_RELEVANT_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

#[derive(Debug)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u8,
    pub attacks: Vec<Bitboard>,
}

pub struct MagicTables {
    pub rook_table: [MagicEntry; 64],
    pub bishop_table: [MagicEntry; 64],
}

impl MagicTables {
    pub fn new() -> Self {
        let mut rook_table = vec![];
        let mut bishop_table = vec![];

        // Initialize rook tables
        for square in 0..64 {
            let sq = Square::from_index(square);
            let mask = rook_mask(sq);
            let magic = ROOK_MAGICS[square as usize];
            let shift = 64 - ROOK_RELEVANT_BITS[square as usize];
            let table_size = 1 << ROOK_RELEVANT_BITS[square as usize];
            
            let mut attacks = vec![Bitboard::EMPTY; table_size];
            
            // Generate all possible occupancy combinations
            let bits = mask.iter().collect::<Vec<_>>();
            for occupancy_index in 0..table_size {
                let mut occupancy = Bitboard::EMPTY;
                for (bit_index, &bit_square) in bits.iter().enumerate() {
                    if (occupancy_index & (1 << bit_index)) != 0 {
                        occupancy |= Bitboard::from_square(bit_square);
                    }
                }
                
                let magic_index = ((occupancy.0.wrapping_mul(magic)) >> shift) as usize;
                attacks[magic_index] = rook_attacks_slow(sq, occupancy);
            }
            
            rook_table.push(MagicEntry {
                mask,
                magic,
                shift,
                attacks,
            });
        }

        // Initialize bishop tables
        for square in 0..64 {
            let sq = Square::from_index(square);
            let mask = bishop_mask(sq);
            let magic = BISHOP_MAGICS[square as usize];
            let shift = 64 - BISHOP_RELEVANT_BITS[square as usize];
            let table_size = 1 << BISHOP_RELEVANT_BITS[square as usize];
            
            let mut attacks = vec![Bitboard::EMPTY; table_size];
            
            // Generate all possible occupancy combinations
            let bits = mask.iter().collect::<Vec<_>>();
            for occupancy_index in 0..table_size {
                let mut occupancy = Bitboard::EMPTY;
                for (bit_index, &bit_square) in bits.iter().enumerate() {
                    if (occupancy_index & (1 << bit_index)) != 0 {
                        occupancy |= Bitboard::from_square(bit_square);
                    }
                }
                
                let magic_index = ((occupancy.0.wrapping_mul(magic)) >> shift) as usize;
                attacks[magic_index] = bishop_attacks_slow(sq, occupancy);
            }
            
            bishop_table.push(MagicEntry {
                mask,
                magic,
                shift,
                attacks,
            });
        }

        MagicTables {
            rook_table: rook_table.try_into().unwrap(),
            bishop_table: bishop_table.try_into().unwrap(),
        }
    }

    #[inline]
    pub fn rook_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let entry = &self.rook_table[square.index()];
        let masked_occupancy = occupancy & entry.mask;
        let magic_index = ((masked_occupancy.0.wrapping_mul(entry.magic)) >> entry.shift) as usize;
        entry.attacks[magic_index]
    }

    #[inline]
    pub fn bishop_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let entry = &self.bishop_table[square.index()];
        let masked_occupancy = occupancy & entry.mask;
        let magic_index = ((masked_occupancy.0.wrapping_mul(entry.magic)) >> entry.shift) as usize;
        entry.attacks[magic_index]
    }

    #[inline]
    pub fn queen_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        self.rook_attacks(square, occupancy) | self.bishop_attacks(square, occupancy)
    }
}

fn rook_mask(square: Square) -> Bitboard {
    let mut mask = Bitboard::EMPTY;
    let file = square.file();
    let rank = square.rank();

    // Horizontal attacks (excluding edges)
    for f in 1..7 {
        if f != file {
            if let Some(sq) = Square::new(f, rank) {
                mask |= Bitboard::from_square(sq);
            }
        }
    }

    // Vertical attacks (excluding edges)
    for r in 1..7 {
        if r != rank {
            if let Some(sq) = Square::new(file, r) {
                mask |= Bitboard::from_square(sq);
            }
        }
    }

    mask
}

fn bishop_mask(square: Square) -> Bitboard {
    let mut mask = Bitboard::EMPTY;
    let file = square.file() as i8;
    let rank = square.rank() as i8;

    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (df, dr) in directions {
        for distance in 1..7 {
            let new_file = file + df * distance;
            let new_rank = rank + dr * distance;

            if new_file > 0 && new_file < 7 && new_rank > 0 && new_rank < 7 {
                if let Some(sq) = Square::new(new_file as u8, new_rank as u8) {
                    mask |= Bitboard::from_square(sq);
                }
            } else {
                break;
            }
        }
    }

    mask
}

fn rook_attacks_slow(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let file = square.file() as i8;
    let rank = square.rank() as i8;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (df, dr) in directions {
        for distance in 1..8 {
            let new_file = file + df * distance;
            let new_rank = rank + dr * distance;

            if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                if let Some(sq) = Square::new(new_file as u8, new_rank as u8) {
                    attacks |= Bitboard::from_square(sq);
                    
                    // Stop if we hit an occupied square
                    if occupancy.is_set(sq) {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    attacks
}

fn bishop_attacks_slow(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let file = square.file() as i8;
    let rank = square.rank() as i8;

    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (df, dr) in directions {
        for distance in 1..8 {
            let new_file = file + df * distance;
            let new_rank = rank + dr * distance;

            if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                if let Some(sq) = Square::new(new_file as u8, new_rank as u8) {
                    attacks |= Bitboard::from_square(sq);
                    
                    // Stop if we hit an occupied square
                    if occupancy.is_set(sq) {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    attacks
}

lazy_static::lazy_static! {
    pub static ref MAGIC_TABLES: MagicTables = MagicTables::new();
}