use crate::chess::{
    Square, Color, Role, Move, MoveList,
    ATTACK_TABLES, MAGIC_TABLES
};

pub struct MoveGenerator;

impl MoveGenerator {
    /// Generate all legal moves for the given position
    pub fn generate_legal_moves(position: &crate::chess::Position) -> MoveList {
        let mut moves = Vec::new();
        
        Self::generate_pawn_moves(position, &mut moves);
        Self::generate_knight_moves(position, &mut moves);
        Self::generate_bishop_moves(position, &mut moves);
        Self::generate_rook_moves(position, &mut moves);
        Self::generate_queen_moves(position, &mut moves);
        Self::generate_king_moves(position, &mut moves);
        Self::generate_castling_moves(position, &mut moves);
        
        // Filter out illegal moves (that would leave king in check)
        moves.into_iter()
             .filter(|m| Self::is_legal_move(position, m))
             .collect()
    }

    /// Generate only capture moves
    pub fn generate_capture_moves(position: &crate::chess::Position) -> MoveList {
        let mut moves = Vec::new();
        
        Self::generate_pawn_captures(position, &mut moves);
        Self::generate_knight_captures(position, &mut moves);
        Self::generate_bishop_captures(position, &mut moves);
        Self::generate_rook_captures(position, &mut moves);
        Self::generate_queen_captures(position, &mut moves);
        Self::generate_king_captures(position, &mut moves);
        
        // Filter out illegal moves
        moves.into_iter()
             .filter(|m| Self::is_legal_move(position, m))
             .collect()
    }

    fn generate_pawn_moves(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let pawns = position.pieces(us, Role::Pawn);
        
        for pawn_square in pawns.iter() {
            Self::generate_pawn_moves_from_square(position, pawn_square, moves);
        }
    }

    fn generate_pawn_captures(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let pawns = position.pieces(us, Role::Pawn);
        
        for pawn_square in pawns.iter() {
            let attacks = ATTACK_TABLES.pawn_attacks[us as usize][pawn_square.index()];
            let captures = attacks & position.enemy_pieces();
            
            for target in captures.iter() {
                let captured_piece = position.piece_at(target).unwrap().role;
                
                // Check for promotion
                let target_rank = target.rank();
                let promotion_rank = if us == Color::White { 7 } else { 0 };
                
                if target_rank == promotion_rank {
                    moves.push(Move::new_promotion(pawn_square, target, Role::Queen, Some(captured_piece)));
                    moves.push(Move::new_promotion(pawn_square, target, Role::Rook, Some(captured_piece)));
                    moves.push(Move::new_promotion(pawn_square, target, Role::Bishop, Some(captured_piece)));
                    moves.push(Move::new_promotion(pawn_square, target, Role::Knight, Some(captured_piece)));
                } else {
                    moves.push(Move::new_capture(pawn_square, target, captured_piece));
                }
            }
            
            // En passant captures
            if let Some(ep_square) = position.en_passant_square() {
                if attacks.is_set(ep_square) {
                    moves.push(Move::new_en_passant(pawn_square, ep_square));
                }
            }
        }
    }

    fn generate_pawn_moves_from_square(position: &crate::chess::Position, square: Square, moves: &mut MoveList) {
        let us = position.side_to_move();
        let empty = position.empty_squares();
        
        // Pawn pushes
        let forward = if us == Color::White {
            square.rank() + 1
        } else {
            square.rank().wrapping_sub(1)
        };
        
        if forward < 8 {
            let target = Square::from_coords(square.file(), forward);
            
            if empty.is_set(target) {
                // Check for promotion
                let promotion_rank = if us == Color::White { 7 } else { 0 };
                
                if forward == promotion_rank {
                    moves.push(Move::new_promotion(square, target, Role::Queen, None));
                    moves.push(Move::new_promotion(square, target, Role::Rook, None));
                    moves.push(Move::new_promotion(square, target, Role::Bishop, None));
                    moves.push(Move::new_promotion(square, target, Role::Knight, None));
                } else {
                    moves.push(Move::new(square, target));
                    
                    // Double pawn push
                    let start_rank = if us == Color::White { 1 } else { 6 };
                    if square.rank() == start_rank {
                        let double_target = Square::from_coords(square.file(), if us == Color::White { 3 } else { 4 });
                        if empty.is_set(double_target) {
                            moves.push(Move::new(square, double_target));
                        }
                    }
                }
            }
        }
        
        // Pawn captures (handled by generate_pawn_captures when called from generate_legal_moves)
    }

    fn generate_knight_moves(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let knights = position.pieces(us, Role::Knight);
        let our_pieces = position.our_pieces();
        
        for knight_square in knights.iter() {
            let attacks = ATTACK_TABLES.knight_attacks[knight_square.index()];
            let targets = attacks & !our_pieces;
            
            for target in targets.iter() {
                if let Some(piece) = position.piece_at(target) {
                    moves.push(Move::new_capture(knight_square, target, piece.role));
                } else {
                    moves.push(Move::new(knight_square, target));
                }
            }
        }
    }

    fn generate_knight_captures(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let knights = position.pieces(us, Role::Knight);
        let enemies = position.enemy_pieces();
        
        for knight_square in knights.iter() {
            let attacks = ATTACK_TABLES.knight_attacks[knight_square.index()];
            let captures = attacks & enemies;
            
            for target in captures.iter() {
                let captured_piece = position.piece_at(target).unwrap().role;
                moves.push(Move::new_capture(knight_square, target, captured_piece));
            }
        }
    }

    fn generate_bishop_moves(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let bishops = position.pieces(us, Role::Bishop);
        let our_pieces = position.our_pieces();
        let all_pieces = position.all_pieces();
        
        for bishop_square in bishops.iter() {
            let attacks = MAGIC_TABLES.bishop_attacks(bishop_square, all_pieces);
            let targets = attacks & !our_pieces;
            
            for target in targets.iter() {
                if let Some(piece) = position.piece_at(target) {
                    moves.push(Move::new_capture(bishop_square, target, piece.role));
                } else {
                    moves.push(Move::new(bishop_square, target));
                }
            }
        }
    }

    fn generate_bishop_captures(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let bishops = position.pieces(us, Role::Bishop);
        let enemies = position.enemy_pieces();
        let all_pieces = position.all_pieces();
        
        for bishop_square in bishops.iter() {
            let attacks = MAGIC_TABLES.bishop_attacks(bishop_square, all_pieces);
            let captures = attacks & enemies;
            
            for target in captures.iter() {
                let captured_piece = position.piece_at(target).unwrap().role;
                moves.push(Move::new_capture(bishop_square, target, captured_piece));
            }
        }
    }

    fn generate_rook_moves(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let rooks = position.pieces(us, Role::Rook);
        let our_pieces = position.our_pieces();
        let all_pieces = position.all_pieces();
        
        for rook_square in rooks.iter() {
            let attacks = MAGIC_TABLES.rook_attacks(rook_square, all_pieces);
            let targets = attacks & !our_pieces;
            
            for target in targets.iter() {
                if let Some(piece) = position.piece_at(target) {
                    moves.push(Move::new_capture(rook_square, target, piece.role));
                } else {
                    moves.push(Move::new(rook_square, target));
                }
            }
        }
    }

    fn generate_rook_captures(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let rooks = position.pieces(us, Role::Rook);
        let enemies = position.enemy_pieces();
        let all_pieces = position.all_pieces();
        
        for rook_square in rooks.iter() {
            let attacks = MAGIC_TABLES.rook_attacks(rook_square, all_pieces);
            let captures = attacks & enemies;
            
            for target in captures.iter() {
                let captured_piece = position.piece_at(target).unwrap().role;
                moves.push(Move::new_capture(rook_square, target, captured_piece));
            }
        }
    }

    fn generate_queen_moves(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let queens = position.pieces(us, Role::Queen);
        let our_pieces = position.our_pieces();
        let all_pieces = position.all_pieces();
        
        for queen_square in queens.iter() {
            let attacks = MAGIC_TABLES.queen_attacks(queen_square, all_pieces);
            let targets = attacks & !our_pieces;
            
            for target in targets.iter() {
                if let Some(piece) = position.piece_at(target) {
                    moves.push(Move::new_capture(queen_square, target, piece.role));
                } else {
                    moves.push(Move::new(queen_square, target));
                }
            }
        }
    }

    fn generate_queen_captures(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let queens = position.pieces(us, Role::Queen);
        let enemies = position.enemy_pieces();
        let all_pieces = position.all_pieces();
        
        for queen_square in queens.iter() {
            let attacks = MAGIC_TABLES.queen_attacks(queen_square, all_pieces);
            let captures = attacks & enemies;
            
            for target in captures.iter() {
                let captured_piece = position.piece_at(target).unwrap().role;
                moves.push(Move::new_capture(queen_square, target, captured_piece));
            }
        }
    }

    fn generate_king_moves(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let king_square = position.king_square(us);
        let our_pieces = position.our_pieces();
        
        let attacks = ATTACK_TABLES.king_attacks[king_square.index()];
        let targets = attacks & !our_pieces;
        
        for target in targets.iter() {
            if let Some(piece) = position.piece_at(target) {
                moves.push(Move::new_capture(king_square, target, piece.role));
            } else {
                moves.push(Move::new(king_square, target));
            }
        }
    }

    fn generate_king_captures(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let king_square = position.king_square(us);
        let enemies = position.enemy_pieces();
        
        let attacks = ATTACK_TABLES.king_attacks[king_square.index()];
        let captures = attacks & enemies;
        
        for target in captures.iter() {
            let captured_piece = position.piece_at(target).unwrap().role;
            moves.push(Move::new_capture(king_square, target, captured_piece));
        }
    }

    fn generate_castling_moves(position: &crate::chess::Position, moves: &mut MoveList) {
        let us = position.side_to_move();
        let rights = position.castling_rights();
        
        if rights.has_king_side(us) {
            if Self::can_castle_king_side(position, us) {
                let king_square = position.king_square(us);
                let target = Square::from_coords(6, king_square.rank());
                moves.push(Move::new_castling(king_square, target));
            }
        }
        
        if rights.has_queen_side(us) {
            if Self::can_castle_queen_side(position, us) {
                let king_square = position.king_square(us);
                let target = Square::from_coords(2, king_square.rank());
                moves.push(Move::new_castling(king_square, target));
            }
        }
    }

    fn can_castle_king_side(position: &crate::chess::Position, color: Color) -> bool {
        let rank = if color == Color::White { 0 } else { 7 };
        let king_square = Square::from_coords(4, rank);
        
        // Check if squares are empty
        let empty_squares = [Square::from_coords(5, rank), Square::from_coords(6, rank)];
        for square in empty_squares {
            if !position.empty_squares().is_set(square) {
                return false;
            }
        }
        
        // Check if king or squares king passes through are under attack
        let check_squares = [king_square, Square::from_coords(5, rank), Square::from_coords(6, rank)];
        for square in check_squares {
            if Self::is_square_attacked(position, square, !color) {
                return false;
            }
        }
        
        true
    }

    fn can_castle_queen_side(position: &crate::chess::Position, color: Color) -> bool {
        let rank = if color == Color::White { 0 } else { 7 };
        let king_square = Square::from_coords(4, rank);
        
        // Check if squares are empty
        let empty_squares = [Square::from_coords(1, rank), Square::from_coords(2, rank), Square::from_coords(3, rank)];
        for square in empty_squares {
            if !position.empty_squares().is_set(square) {
                return false;
            }
        }
        
        // Check if king or squares king passes through are under attack
        let check_squares = [king_square, Square::from_coords(2, rank), Square::from_coords(3, rank)];
        for square in check_squares {
            if Self::is_square_attacked(position, square, !color) {
                return false;
            }
        }
        
        true
    }

    fn is_legal_move(position: &crate::chess::Position, mov: &Move) -> bool {
        // Make the move temporarily and check if our king is in check
        let mut temp_position = *position;
        temp_position.make_move_unchecked(mov);
        
        let our_king = temp_position.king_square(position.side_to_move());
        !Self::is_square_attacked(&temp_position, our_king, !position.side_to_move())
    }

    pub fn is_square_attacked(position: &crate::chess::Position, square: Square, by_color: Color) -> bool {
        let all_pieces = position.all_pieces();
        
        // Check pawn attacks
        let pawn_attacks = ATTACK_TABLES.pawn_attacks[(!by_color) as usize][square.index()];
        if (pawn_attacks & position.pieces(by_color, Role::Pawn)).pop_count() > 0 {
            return true;
        }
        
        // Check knight attacks
        let knight_attacks = ATTACK_TABLES.knight_attacks[square.index()];
        if (knight_attacks & position.pieces(by_color, Role::Knight)).pop_count() > 0 {
            return true;
        }
        
        // Check bishop/queen diagonal attacks
        let bishop_attacks = MAGIC_TABLES.bishop_attacks(square, all_pieces);
        let bishop_attackers = position.pieces(by_color, Role::Bishop) | position.pieces(by_color, Role::Queen);
        if (bishop_attacks & bishop_attackers).pop_count() > 0 {
            return true;
        }
        
        // Check rook/queen straight attacks
        let rook_attacks = MAGIC_TABLES.rook_attacks(square, all_pieces);
        let rook_attackers = position.pieces(by_color, Role::Rook) | position.pieces(by_color, Role::Queen);
        if (rook_attacks & rook_attackers).pop_count() > 0 {
            return true;
        }
        
        // Check king attacks
        let king_attacks = ATTACK_TABLES.king_attacks[square.index()];
        if (king_attacks & position.pieces(by_color, Role::King)).pop_count() > 0 {
            return true;
        }
        
        false
    }
}