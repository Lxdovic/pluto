#[cfg(test)]
mod debug_perft {
    use crate::chess::{Position, PerftTest};

    #[test]
    fn debug_perft_3() {
        let position = Position::startpos();
        
        println!("Running perft divide for depth 3:");
        let result = PerftTest::perft_divide(&position, 3);
        println!("Total: {}", result);
        
        // Let's also check depth 2 for reference
        println!("\nRunning perft divide for depth 2:");
        let result2 = PerftTest::perft_divide(&position, 2);
        println!("Total: {}", result2);
    }
}