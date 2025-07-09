use engine::chess::{test_basic_setup, Position, PerftTest};

fn main() {
    println!("Testing custom chess implementation...");
    
    // Test basic setup
    test_basic_setup();
    
    // Test position creation
    let position = Position::startpos();
    println!("Starting position created successfully");
    println!("Side to move: {:?}", position.side_to_move());
    println!("All pieces count: {}", position.all_pieces().pop_count());
    
    // Try to generate some basic moves (this might fail initially)
    match std::panic::catch_unwind(|| {
        let moves = position.legal_moves();
        println!("Generated {} legal moves", moves.len());
        
        if moves.len() > 0 {
            println!("First few moves:");
            for (i, mv) in moves.iter().enumerate().take(5) {
                println!("  {}: {:?}", i + 1, mv);
            }
        }
    }) {
        Ok(_) => {
            println!("Move generation worked!");
            
            // Try a simple perft test
            println!("\nTrying perft test...");
            match std::panic::catch_unwind(|| {
                let result = PerftTest::perft(&position, 1);
                println!("Perft(1) = {}", result);
                result
            }) {
                Ok(result) => {
                    if result == 20 {
                        println!("Perft(1) correct!");
                    } else {
                        println!("Perft(1) incorrect, expected 20, got {}", result);
                    }
                }
                Err(_) => println!("Perft test failed with panic"),
            }
        }
        Err(_) => println!("Move generation failed with panic"),
    }
}