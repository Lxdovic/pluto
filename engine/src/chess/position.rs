use crate::chess::bitboard::Bitboard;
use crate::chess::types::{Square, Color, Role, Piece, Move, MoveType, MoveList, CastlingRights};
use crate::chess::move_gen::MoveGenerator;
use crate::chess::zobrist::Zobrist64;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    // Piece bitboards
    piece_bitboards: [[Bitboard; 6]; 2], // [color][piece_type]
    color_bitboards: [Bitboard; 2],      // [color]
    all_pieces: Bitboard,
    
    // Game state
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    
    // Zobrist hash
    hash: Zobrist64,
}

impl Position {
    /// Create a new position from the starting position
    pub fn startpos() -> Self {
        let mut position = Position {
            piece_bitboards: [[Bitboard::EMPTY; 6]; 2],
            color_bitboards: [Bitboard::EMPTY; 2],
            all_pieces: Bitboard::EMPTY,
            side_to_move: Color::White,
            castling_rights: CastlingRights::new(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            hash: Zobrist64::default(),
        };

        // Set up starting position
        position.setup_starting_position();
        position.compute_hash();
        position
    }

    /// Create an empty position
    pub fn empty() -> Self {
        Position {
            piece_bitboards: [[Bitboard::EMPTY; 6]; 2],
            color_bitboards: [Bitboard::EMPTY; 2],
            all_pieces: Bitboard::EMPTY,
            side_to_move: Color::White,
            castling_rights: CastlingRights::empty(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            hash: Zobrist64::default(),
        }
    }

    fn setup_starting_position(&mut self) {
        // Place white pieces
        self.place_piece(Piece::new(Color::White, Role::Rook), Square::A1);
        self.place_piece(Piece::new(Color::White, Role::Knight), Square::B1);
        self.place_piece(Piece::new(Color::White, Role::Bishop), Square::C1);
        self.place_piece(Piece::new(Color::White, Role::Queen), Square::D1);
        self.place_piece(Piece::new(Color::White, Role::King), Square::E1);
        self.place_piece(Piece::new(Color::White, Role::Bishop), Square::F1);
        self.place_piece(Piece::new(Color::White, Role::Knight), Square::G1);
        self.place_piece(Piece::new(Color::White, Role::Rook), Square::H1);

        for file in 0..8 {
            self.place_piece(Piece::new(Color::White, Role::Pawn), Square::from_coords(file, 1));
        }

        // Place black pieces
        self.place_piece(Piece::new(Color::Black, Role::Rook), Square::A8);
        self.place_piece(Piece::new(Color::Black, Role::Knight), Square::B8);
        self.place_piece(Piece::new(Color::Black, Role::Bishop), Square::C8);
        self.place_piece(Piece::new(Color::Black, Role::Queen), Square::D8);
        self.place_piece(Piece::new(Color::Black, Role::King), Square::E8);
        self.place_piece(Piece::new(Color::Black, Role::Bishop), Square::F8);
        self.place_piece(Piece::new(Color::Black, Role::Knight), Square::G8);
        self.place_piece(Piece::new(Color::Black, Role::Rook), Square::H8);

        for file in 0..8 {
            self.place_piece(Piece::new(Color::Black, Role::Pawn), Square::from_coords(file, 6));
        }

        self.update_composite_bitboards();
    }

    pub fn place_piece(&mut self, piece: Piece, square: Square) {
        let color_idx = piece.color as usize;
        let piece_idx = (piece.role as usize) - 1;
        
        self.piece_bitboards[color_idx][piece_idx].set(square);
    }

    fn remove_piece(&mut self, square: Square) -> Option<Piece> {
        for color in [Color::White, Color::Black] {
            for role in [Role::Pawn, Role::Knight, Role::Bishop, Role::Rook, Role::Queen, Role::King] {
                let color_idx = color as usize;
                let piece_idx = (role as usize) - 1;
                
                if self.piece_bitboards[color_idx][piece_idx].is_set(square) {
                    self.piece_bitboards[color_idx][piece_idx].clear(square);
                    return Some(Piece::new(color, role));
                }
            }
        }
        None
    }

    pub fn update_composite_bitboards(&mut self) {
        self.color_bitboards[Color::White as usize] = Bitboard::EMPTY;
        self.color_bitboards[Color::Black as usize] = Bitboard::EMPTY;

        for color in [Color::White, Color::Black] {
            let color_idx = color as usize;
            for piece_idx in 0..6 {
                self.color_bitboards[color_idx] |= self.piece_bitboards[color_idx][piece_idx];
            }
        }

        self.all_pieces = self.color_bitboards[0] | self.color_bitboards[1];
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn pieces(&self, color: Color, role: Role) -> Bitboard {
        self.piece_bitboards[color as usize][(role as usize) - 1]
    }

    pub fn our_pieces(&self) -> Bitboard {
        self.color_bitboards[self.side_to_move as usize]
    }

    pub fn enemy_pieces(&self) -> Bitboard {
        self.color_bitboards[(!self.side_to_move) as usize]
    }

    pub fn all_pieces(&self) -> Bitboard {
        self.all_pieces
    }

    pub fn empty_squares(&self) -> Bitboard {
        !self.all_pieces
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        for color in [Color::White, Color::Black] {
            for role in [Role::Pawn, Role::Knight, Role::Bishop, Role::Rook, Role::Queen, Role::King] {
                if self.pieces(color, role).is_set(square) {
                    return Some(Piece::new(color, role));
                }
            }
        }
        None
    }

    pub fn king_square(&self, color: Color) -> Square {
        self.pieces(color, Role::King).lsb().expect("King must be present")
    }

    pub fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    pub fn legal_moves(&self) -> MoveList {
        MoveGenerator::generate_legal_moves(self)
    }

    pub fn capture_moves(&self) -> MoveList {
        MoveGenerator::generate_capture_moves(self)
    }

    pub fn is_check(&self) -> bool {
        let king_square = self.king_square(self.side_to_move);
        MoveGenerator::is_square_attacked(self, king_square, !self.side_to_move)
    }

    pub fn is_checkmate(&self) -> bool {
        self.is_check() && self.legal_moves().is_empty()
    }

    pub fn is_stalemate(&self) -> bool {
        !self.is_check() && self.legal_moves().is_empty()
    }

    pub fn make_move(&mut self, mov: &Move) -> bool {
        if self.legal_moves().contains(mov) {
            self.make_move_unchecked(mov);
            true
        } else {
            false
        }
    }

    pub fn make_move_unchecked(&mut self, mov: &Move) {
        // Get the piece being moved
        let piece = self.piece_at(mov.from).expect("Moving piece must exist");
        
        // Remove piece from origin
        self.remove_piece(mov.from);
        
        // Handle captures
        if mov.is_capture() && !mov.is_en_passant() {
            self.remove_piece(mov.to);
        }
        
        match mov.move_type {
            MoveType::Normal => {
                self.place_piece(piece, mov.to);
            }
            
            MoveType::Promotion { piece: promoted_role } => {
                let promoted_piece = Piece::new(piece.color, promoted_role);
                self.place_piece(promoted_piece, mov.to);
            }
            
            MoveType::EnPassant => {
                self.place_piece(piece, mov.to);
                // Remove the captured pawn
                let captured_pawn_square = Square::from_coords(
                    mov.to.file(),
                    if self.side_to_move == Color::White { mov.to.rank() - 1 } else { mov.to.rank() + 1 }
                );
                self.remove_piece(captured_pawn_square);
            }
            
            MoveType::Castling => {
                self.place_piece(piece, mov.to);
                
                // Move the rook
                let (rook_from, rook_to) = if mov.to.file() == 6 {
                    // King-side castling
                    (Square::from_coords(7, mov.from.rank()), Square::from_coords(5, mov.from.rank()))
                } else {
                    // Queen-side castling
                    (Square::from_coords(0, mov.from.rank()), Square::from_coords(3, mov.from.rank()))
                };
                
                let rook = self.remove_piece(rook_from).expect("Rook must be present for castling");
                self.place_piece(rook, rook_to);
            }
        }
        
        // Update castling rights
        self.update_castling_rights(mov);
        
        // Update en passant square
        self.update_en_passant_square(mov, piece);
        
        // Update move counters
        if mov.is_capture() || piece.role == Role::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
        
        if self.side_to_move == Color::Black {
            self.fullmove_number += 1;
        }
        
        // Switch side to move
        self.side_to_move = !self.side_to_move;
        
        // Update composite bitboards
        self.update_composite_bitboards();
        
        // Update hash
        self.compute_hash();
    }

    fn update_castling_rights(&mut self, mov: &Move) {
        let piece = self.piece_at(mov.from);
        
        // If king moves, remove all castling rights for that color
        if let Some(piece) = piece {
            if piece.role == Role::King {
                self.castling_rights.remove_king_side(piece.color);
                self.castling_rights.remove_queen_side(piece.color);
                return;
            }
        }
        
        // If rook moves from starting square, remove appropriate castling right
        match mov.from {
            Square::A1 => self.castling_rights.remove_queen_side(Color::White),
            Square::H1 => self.castling_rights.remove_king_side(Color::White),
            Square::A8 => self.castling_rights.remove_queen_side(Color::Black),
            Square::H8 => self.castling_rights.remove_king_side(Color::Black),
            _ => {}
        }
        
        // If rook is captured on starting square, remove appropriate castling right
        match mov.to {
            Square::A1 => self.castling_rights.remove_queen_side(Color::White),
            Square::H1 => self.castling_rights.remove_king_side(Color::White),
            Square::A8 => self.castling_rights.remove_queen_side(Color::Black),
            Square::H8 => self.castling_rights.remove_king_side(Color::Black),
            _ => {}
        }
    }

    fn update_en_passant_square(&mut self, mov: &Move, piece: Piece) {
        self.en_passant_square = None;
        
        // Set en passant square for double pawn pushes
        if piece.role == Role::Pawn {
            let rank_diff = (mov.to.rank() as i8 - mov.from.rank() as i8).abs();
            if rank_diff == 2 {
                let ep_rank = (mov.from.rank() + mov.to.rank()) / 2;
                self.en_passant_square = Some(Square::from_coords(mov.from.file(), ep_rank));
            }
        }
    }

    pub fn zobrist_hash(&self) -> Zobrist64 {
        self.hash
    }

    pub fn compute_hash(&mut self) {
        // This is a placeholder - full Zobrist hashing will be implemented in zobrist.rs
        self.hash = Zobrist64::default();
    }

    pub fn clone(&self) -> Self {
        *self
    }

    pub fn play(&self, mov: &Move) -> Option<Self> {
        if self.legal_moves().contains(mov) {
            let mut new_pos = *self;
            new_pos.make_move_unchecked(mov);
            Some(new_pos)
        } else {
            None
        }
    }

    pub fn play_unchecked(&self, mov: &Move) -> Self {
        let mut new_pos = *self;
        new_pos.make_move_unchecked(mov);
        new_pos
    }

    pub fn turn(&self) -> Color {
        self.side_to_move
    }

    pub fn board(&self) -> &Self {
        self
    }

    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    pub fn fullmove_number(&self) -> u16 {
        self.fullmove_number
    }

    pub fn from_fen(fen: &str) -> Result<Self, crate::chess::fen::FenError> {
        crate::chess::fen::Fen::new(fen).into_position()
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::startpos()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}