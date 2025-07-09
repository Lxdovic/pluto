#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    #[inline]
    pub fn flip(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn pawn(self) -> Piece {
        Piece { color: self, role: Role::Pawn }
    }

    #[inline]
    pub fn rook(self) -> Piece {
        Piece { color: self, role: Role::Rook }
    }

    #[inline]
    pub fn knight(self) -> Piece {
        Piece { color: self, role: Role::Knight }
    }

    #[inline]
    pub fn bishop(self) -> Piece {
        Piece { color: self, role: Role::Bishop }
    }

    #[inline]
    pub fn queen(self) -> Piece {
        Piece { color: self, role: Role::Queen }
    }

    #[inline]
    pub fn king(self) -> Piece {
        Piece { color: self, role: Role::King }
    }
}

impl std::ops::Not for Color {
    type Output = Color;
    
    #[inline]
    fn not(self) -> Color {
        self.flip()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Role {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    #[inline]
    pub fn new(color: Color, role: Role) -> Self {
        Self { color, role }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    #[inline]
    pub fn new(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(unsafe { std::mem::transmute(rank * 8 + file) })
        } else {
            None
        }
    }

    #[inline]
    pub fn from_coords(file: u8, rank: u8) -> Self {
        Self::new(file, rank).unwrap()
    }

    #[inline]
    pub fn file(self) -> u8 {
        (self as u8) % 8
    }

    #[inline]
    pub fn rank(self) -> u8 {
        (self as u8) / 8
    }

    #[inline]
    pub fn flip_vertical(self) -> Self {
        unsafe { std::mem::transmute((7 - self.rank()) * 8 + self.file()) }
    }

    #[inline]
    pub fn from_index(index: u8) -> Self {
        unsafe { std::mem::transmute(index) }
    }

    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }
}

impl From<u8> for Square {
    #[inline]
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    Promotion { piece: Role },
    EnPassant,
    Castling,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
    pub captured_piece: Option<Role>,
}

impl Move {
    #[inline]
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Normal,
            captured_piece: None,
        }
    }

    #[inline]
    pub fn new_capture(from: Square, to: Square, captured: Role) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Normal,
            captured_piece: Some(captured),
        }
    }

    #[inline]
    pub fn new_promotion(from: Square, to: Square, piece: Role, captured: Option<Role>) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Promotion { piece },
            captured_piece: captured,
        }
    }

    #[inline]
    pub fn new_en_passant(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::EnPassant,
            captured_piece: Some(Role::Pawn),
        }
    }

    #[inline]
    pub fn new_castling(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Castling,
            captured_piece: None,
        }
    }

    #[inline]
    pub fn is_capture(&self) -> bool {
        self.captured_piece.is_some()
    }

    #[inline]
    pub fn is_promotion(&self) -> bool {
        matches!(self.move_type, MoveType::Promotion { .. })
    }

    #[inline]
    pub fn is_en_passant(&self) -> bool {
        matches!(self.move_type, MoveType::EnPassant)
    }

    #[inline]
    pub fn is_castling(&self) -> bool {
        matches!(self.move_type, MoveType::Castling)
    }

    #[inline]
    pub fn promotion_piece(&self) -> Option<Role> {
        match self.move_type {
            MoveType::Promotion { piece } => Some(piece),
            _ => None,
        }
    }

    #[inline]
    pub fn capture(&self) -> Option<Role> {
        self.captured_piece
    }

    #[inline]
    pub fn role(&self) -> Role {
        // This is used for the piece being moved - will be determined from position
        Role::Pawn // placeholder, will be filled by position context
    }
}

pub type MoveList = Vec<Move>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl CastlingRights {
    #[inline]
    pub fn new() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }

    #[inline]
    pub fn has_king_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_king_side,
            Color::Black => self.black_king_side,
        }
    }

    #[inline]
    pub fn has_queen_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_queen_side,
            Color::Black => self.black_queen_side,
        }
    }

    #[inline]
    pub fn remove_king_side(&mut self, color: Color) {
        match color {
            Color::White => self.white_king_side = false,
            Color::Black => self.black_king_side = false,
        }
    }

    #[inline]
    pub fn remove_queen_side(&mut self, color: Color) {
        match color {
            Color::White => self.white_queen_side = false,
            Color::Black => self.black_queen_side = false,
        }
    }
}