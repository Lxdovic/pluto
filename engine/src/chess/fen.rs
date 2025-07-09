use crate::chess::position::Position;
use crate::chess::types::{Square, Color, Role, Piece, CastlingRights};

#[derive(Debug)]
pub enum FenError {
    InvalidFormat,
    InvalidPiece(char),
    InvalidSquare,
    InvalidCastlingRights,
    InvalidEnPassant,
    InvalidSideToMove,
    InvalidClock,
}

pub struct Fen(pub String);

impl Fen {
    pub fn new(fen: &str) -> Self {
        Self(fen.to_string())
    }

    pub fn into_position(self) -> Result<Position, FenError> {
        let parts: Vec<&str> = self.0.split_whitespace().collect();
        
        if parts.len() < 4 {
            return Err(FenError::InvalidFormat);
        }

        let mut position = Position::empty();

        // Parse piece placement
        self.parse_piece_placement(parts[0], &mut position)?;

        // Parse side to move
        position.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError::InvalidSideToMove),
        };

        // Parse castling rights
        position.castling_rights = self.parse_castling_rights(parts[2])?;

        // Parse en passant square
        position.en_passant_square = self.parse_en_passant(parts[3])?;

        // Parse halfmove clock (optional)
        if parts.len() > 4 {
            position.halfmove_clock = parts[4].parse().map_err(|_| FenError::InvalidClock)?;
        }

        // Parse fullmove number (optional)
        if parts.len() > 5 {
            position.fullmove_number = parts[5].parse().map_err(|_| FenError::InvalidClock)?;
        }

        position.update_composite_bitboards();
        position.compute_hash();

        Ok(position)
    }

    fn parse_piece_placement(&self, placement: &str, position: &mut Position) -> Result<(), FenError> {
        let ranks: Vec<&str> = placement.split('/').collect();
        if ranks.len() != 8 {
            return Err(FenError::InvalidFormat);
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - rank_idx; // FEN starts from rank 8
            let mut file = 0;

            for ch in rank_str.chars() {
                if ch.is_ascii_digit() {
                    file += ch.to_digit(10).unwrap() as u8;
                } else {
                    let piece = self.char_to_piece(ch)?;
                    let square = Square::from_coords(file, rank as u8);
                    position.place_piece(piece, square);
                    file += 1;
                }
                
                if file > 8 {
                    return Err(FenError::InvalidFormat);
                }
            }
            
            if file != 8 {
                return Err(FenError::InvalidFormat);
            }
        }

        Ok(())
    }

    fn char_to_piece(&self, ch: char) -> Result<Piece, FenError> {
        let color = if ch.is_uppercase() { Color::White } else { Color::Black };
        let role = match ch.to_ascii_lowercase() {
            'p' => Role::Pawn,
            'n' => Role::Knight,
            'b' => Role::Bishop,
            'r' => Role::Rook,
            'q' => Role::Queen,
            'k' => Role::King,
            _ => return Err(FenError::InvalidPiece(ch)),
        };
        Ok(Piece::new(color, role))
    }

    fn parse_castling_rights(&self, rights_str: &str) -> Result<CastlingRights, FenError> {
        if rights_str == "-" {
            return Ok(CastlingRights::empty());
        }

        let mut rights = CastlingRights::empty();
        for ch in rights_str.chars() {
            match ch {
                'K' => rights.white_king_side = true,
                'Q' => rights.white_queen_side = true,
                'k' => rights.black_king_side = true,
                'q' => rights.black_queen_side = true,
                _ => return Err(FenError::InvalidCastlingRights),
            }
        }

        Ok(rights)
    }

    fn parse_en_passant(&self, ep_str: &str) -> Result<Option<Square>, FenError> {
        if ep_str == "-" {
            return Ok(None);
        }

        if ep_str.len() != 2 {
            return Err(FenError::InvalidEnPassant);
        }

        let chars: Vec<char> = ep_str.chars().collect();
        let file = match chars[0] {
            'a'..='h' => chars[0] as u8 - b'a',
            _ => return Err(FenError::InvalidEnPassant),
        };
        let rank = match chars[1] {
            '1'..='8' => chars[1] as u8 - b'1',
            _ => return Err(FenError::InvalidEnPassant),
        };

        Ok(Some(Square::from_coords(file, rank)))
    }
}

impl std::str::FromStr for Fen {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fen::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = Position::from_fen(fen).unwrap();
        
        assert_eq!(position.side_to_move(), Color::White);
        assert_eq!(position.all_pieces().pop_count(), 32);
        
        // Test specific pieces
        assert_eq!(position.piece_at(Square::E1).unwrap().role, Role::King);
        assert_eq!(position.piece_at(Square::E1).unwrap().color, Color::White);
        assert_eq!(position.piece_at(Square::E8).unwrap().role, Role::King);
        assert_eq!(position.piece_at(Square::E8).unwrap().color, Color::Black);
    }

    #[test]
    fn test_custom_position_fen() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
        let position = Position::from_fen(fen).unwrap();
        
        assert_eq!(position.all_pieces().pop_count(), 6);
        assert_eq!(position.castling_rights().white_king_side, true);
        assert_eq!(position.castling_rights().white_queen_side, true);
        assert_eq!(position.castling_rights().black_king_side, true);
        assert_eq!(position.castling_rights().black_queen_side, true);
    }
}