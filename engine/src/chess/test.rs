use crate::chess::position::Position;
use crate::chess::types::{Square, Color, Role};

pub fn test_basic_setup() {
    println!("Testing basic position setup...");
    
    let position = Position::startpos();
    
    // Test white pieces
    assert_eq!(position.pieces(Color::White, Role::Pawn).pop_count(), 8);
    assert_eq!(position.pieces(Color::White, Role::Rook).pop_count(), 2);
    assert_eq!(position.pieces(Color::White, Role::Knight).pop_count(), 2);
    assert_eq!(position.pieces(Color::White, Role::Bishop).pop_count(), 2);
    assert_eq!(position.pieces(Color::White, Role::Queen).pop_count(), 1);
    assert_eq!(position.pieces(Color::White, Role::King).pop_count(), 1);
    
    // Test black pieces
    assert_eq!(position.pieces(Color::Black, Role::Pawn).pop_count(), 8);
    assert_eq!(position.pieces(Color::Black, Role::Rook).pop_count(), 2);
    assert_eq!(position.pieces(Color::Black, Role::Knight).pop_count(), 2);
    assert_eq!(position.pieces(Color::Black, Role::Bishop).pop_count(), 2);
    assert_eq!(position.pieces(Color::Black, Role::Queen).pop_count(), 1);
    assert_eq!(position.pieces(Color::Black, Role::King).pop_count(), 1);
    
    // Test specific piece placements
    assert!(position.piece_at(Square::E1).is_some());
    assert_eq!(position.piece_at(Square::E1).unwrap().role, Role::King);
    assert_eq!(position.piece_at(Square::E1).unwrap().color, Color::White);
    
    assert!(position.piece_at(Square::E8).is_some());
    assert_eq!(position.piece_at(Square::E8).unwrap().role, Role::King);
    assert_eq!(position.piece_at(Square::E8).unwrap().color, Color::Black);
    
    println!("Basic setup test passed!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::position::Position;
    use crate::chess::types::{Color, Role, Square};

    #[test]
    fn test_position_creation() {
        let position = Position::startpos();
        assert_eq!(position.side_to_move(), Color::White);
        assert_eq!(position.all_pieces().pop_count(), 32);
    }

    #[test]
    fn test_piece_placement() {
        let position = Position::startpos();
        
        // Test white king
        let white_king = position.piece_at(Square::E1);
        assert!(white_king.is_some());
        assert_eq!(white_king.unwrap().color, Color::White);
        assert_eq!(white_king.unwrap().role, Role::King);
        
        // Test black king
        let black_king = position.piece_at(Square::E8);
        assert!(black_king.is_some());
        assert_eq!(black_king.unwrap().color, Color::Black);
        assert_eq!(black_king.unwrap().role, Role::King);
        
        // Test empty square
        let empty = position.piece_at(Square::E4);
        assert!(empty.is_none());
    }

    #[test]
    fn test_bitboard_operations() {
        let position = Position::startpos();
        
        // Test piece counts
        assert_eq!(position.pieces(Color::White, Role::Pawn).pop_count(), 8);
        assert_eq!(position.pieces(Color::Black, Role::Pawn).pop_count(), 8);
        
        // Test composite bitboards
        assert_eq!(position.our_pieces().pop_count(), 16); // White pieces
        assert_eq!(position.enemy_pieces().pop_count(), 16); // Black pieces
        assert_eq!(position.all_pieces().pop_count(), 32);
        assert_eq!(position.empty_squares().pop_count(), 32);
    }
}