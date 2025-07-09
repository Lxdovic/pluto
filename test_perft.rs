use engine::chess::{Position, PerftTest};

fn main() {
    println!("Testing perft implementation...");
    
    let position = Position::startpos();
    println!("Starting position created");
    
    // Test basic perft
    for depth in 1..=4 {
        let start = std::time::Instant::now();
        let result = PerftTest::perft(&position, depth);
        let elapsed = start.elapsed();
        
        let expected = match depth {
            1 => 20,
            2 => 400,
            3 => 8902,
            4 => 197281,
            _ => 0,
        };
        
        let status = if result == expected { "PASS" } else { "FAIL" };
        println!("Perft({}) = {} [{}] (expected: {}) in {:?}", 
                 depth, result, status, expected, elapsed);
                 
        if result != expected {
            println!("FAILURE: Got {} instead of {} at depth {}", result, expected, depth);
            break;
        }
    }
}