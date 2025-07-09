pub mod bitboard;
pub mod fen;
pub mod magics;
pub mod move_gen;
pub mod perft;
pub mod position;
pub mod test;
pub mod types;
pub mod uci_move;
pub mod zobrist;

pub use bitboard::*;
pub use magics::*;
pub use move_gen::*;
pub use zobrist::Zobrist64;

// Export specific items to avoid conflicts
pub use position::Position;
pub use types::{Move, Color, Square, Piece, Role, MoveList, CastlingRights, MoveType};
pub use fen::Fen;
pub use uci_move::UciMove;
pub use perft::PerftTest;
pub use test::test_basic_setup;