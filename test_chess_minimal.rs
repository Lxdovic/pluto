// Test just the chess module by creating a minimal standalone test

mod bitboard;
mod fen;
mod magics;
mod move_gen;
mod perft;
mod position;
mod test;
mod types;
mod uci_move;
mod zobrist;

use crate::position::Position;
use crate::perft::PerftTest;

fn main() {
    println!("Testing perft implementation...");
    
    let position = Position::startpos();
    println!("Starting position created");
    
    // Get a sample of moves to check
    let moves = position.legal_moves();
    println!("Generated {} legal moves at depth 0", moves.len());
    
    if moves.len() > 0 {
        println!("First few moves:");
        for (i, mv) in moves.iter().enumerate().take(5) {
            println!("  {}: {:?}", i + 1, mv);
        }
    }
    
    // Test basic perft
    for depth in 1..=3 {
        let start = std::time::Instant::now();
        let result = PerftTest::perft(&position, depth);
        let elapsed = start.elapsed();
        
        let expected = match depth {
            1 => 20,
            2 => 400,
            3 => 8902,
            _ => 0,
        };
        
        let status = if result == expected { "PASS" } else { "FAIL" };
        println!("Perft({}) = {} [{}] (expected: {}) in {:?}", 
                 depth, result, status, expected, elapsed);
                 
        if result != expected {
            println!("FAILURE: Got {} instead of {} at depth {}", result, expected, depth);
            
            // Debug with perft divide if depth is small
            if depth <= 2 {
                println!("Running perft divide to debug:");
                PerftTest::perft_divide(&position, depth);
            }
            break;
        }
    }
}