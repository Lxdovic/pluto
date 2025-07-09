use crate::types::{Move, Square, Role};
use crate::position::Position;

#[derive(Debug)]
pub enum UciMoveError {
    InvalidFormat,
    InvalidSquare,
    InvalidPromotion,
}

pub struct UciMove(pub String);

impl UciMove {
    pub fn new(move_str: &str) -> Self {
        Self(move_str.to_string())
    }

    pub fn to_move(&self, position: &Position) -> Result<Move, UciMoveError> {
        let move_str = &self.0;
        
        if move_str.len() < 4 || move_str.len() > 5 {
            return Err(UciMoveError::InvalidFormat);
        }

        let from_str = &move_str[0..2];
        let to_str = &move_str[2..4];
        
        let from = self.parse_square(from_str)?;
        let to = self.parse_square(to_str)?;

        // Check if it's a promotion
        if move_str.len() == 5 {
            let promotion_char = move_str.chars().nth(4).unwrap();
            let promotion_piece = match promotion_char {
                'q' => Role::Queen,
                'r' => Role::Rook,
                'b' => Role::Bishop,
                'n' => Role::Knight,
                _ => return Err(UciMoveError::InvalidPromotion),
            };

            let captured = position.piece_at(to).map(|p| p.role);
            return Ok(Move::new_promotion(from, to, promotion_piece, captured));
        }

        // Check for special moves based on the position
        let piece = position.piece_at(from).ok_or(UciMoveError::InvalidFormat)?;
        
        // Check for castling
        if piece.role == Role::King && (from.file() as i8 - to.file() as i8).abs() == 2 {
            return Ok(Move::new_castling(from, to));
        }

        // Check for en passant
        if piece.role == Role::Pawn {
            if let Some(ep_square) = position.en_passant_square() {
                if to == ep_square {
                    return Ok(Move::new_en_passant(from, to));
                }
            }
        }

        // Regular move or capture
        if let Some(captured_piece) = position.piece_at(to) {
            Ok(Move::new_capture(from, to, captured_piece.role))
        } else {
            Ok(Move::new(from, to))
        }
    }

    fn parse_square(&self, square_str: &str) -> Result<Square, UciMoveError> {
        if square_str.len() != 2 {
            return Err(UciMoveError::InvalidSquare);
        }

        let chars: Vec<char> = square_str.chars().collect();
        let file = match chars[0] {
            'a'..='h' => chars[0] as u8 - b'a',
            _ => return Err(UciMoveError::InvalidSquare),
        };
        let rank = match chars[1] {
            '1'..='8' => chars[1] as u8 - b'1',
            _ => return Err(UciMoveError::InvalidSquare),
        };

        Ok(Square::from_coords(file, rank))
    }

    pub fn to_string(mov: &Move) -> String {
        let from_str = Self::square_to_string(mov.from);
        let to_str = Self::square_to_string(mov.to);
        
        match mov.promotion_piece() {
            Some(piece) => {
                let promotion_char = match piece {
                    Role::Queen => 'q',
                    Role::Rook => 'r',
                    Role::Bishop => 'b',
                    Role::Knight => 'n',
                    _ => 'q', // fallback
                };
                format!("{}{}{}", from_str, to_str, promotion_char)
            }
            None => format!("{}{}", from_str, to_str)
        }
    }

    fn square_to_string(square: Square) -> String {
        let file = (b'a' + square.file()) as char;
        let rank = (b'1' + square.rank()) as char;
        format!("{}{}", file, rank)
    }
}

impl std::str::FromStr for UciMove {
    type Err = UciMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 || s.len() > 5 {
            return Err(UciMoveError::InvalidFormat);
        }
        Ok(UciMove::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    #[test]
    fn test_parse_normal_move() {
        let position = Position::startpos();
        let uci_move = UciMove::new("e2e4");
        let mov = uci_move.to_move(&position).unwrap();
        
        assert_eq!(mov.from, Square::E2);
        assert_eq!(mov.to, Square::E4);
        assert!(!mov.is_capture());
    }

    #[test]
    fn test_uci_move_to_string() {
        let position = Position::startpos();
        let uci_move = UciMove::new("e2e4");
        let mov = uci_move.to_move(&position).unwrap();
        
        assert_eq!(UciMove::to_string(&mov), "e2e4");
    }

    #[test]
    fn test_promotion_move() {
        let position = Position::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let uci_move = UciMove::new("a7a8q");
        let mov = uci_move.to_move(&position).unwrap();
        
        assert!(mov.is_promotion());
        assert_eq!(mov.promotion_piece(), Some(Role::Queen));
    }
}