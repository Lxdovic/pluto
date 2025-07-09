// Simple perft test
extern crate engine;
use engine::chess::{Position, PerftTest};

fn main() {
    println!("Running perft tests...");
    
    let pos = Position::startpos();
    
    for depth in 1..=3 {
        let result = PerftTest::perft(&pos, depth);
        let expected = match depth {
            1 => 20,
            2 => 400,
            3 => 8902,
            _ => 0,
        };
        
        println!("Perft({}) = {} (expected {})", depth, result, expected);
        
        if result != expected {
            println!("FAIL at depth {}: {} != {}", depth, result, expected);
            if depth == 1 {
                println!("Debug: perft divide");
                PerftTest::perft_divide(&pos, 1);
            }
            break;
        } else {
            println!("PASS");
        }
    }
}