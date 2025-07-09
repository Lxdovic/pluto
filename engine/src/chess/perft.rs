use crate::chess::position::Position;

pub struct PerftTest;

impl PerftTest {
    /// Perft (performance test) - counts the number of leaf nodes at a given depth
    pub fn perft(position: &Position, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = position.legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }

        let mut count = 0;
        for mv in moves {
            if let Some(new_pos) = position.play(&mv) {
                count += Self::perft(&new_pos, depth - 1);
            }
        }

        count
    }

    /// Perft with move breakdown for debugging
    pub fn perft_divide(position: &Position, depth: u8) -> u64 {
        let moves = position.legal_moves();
        let mut total = 0;

        for mv in moves {
            if let Some(new_pos) = position.play(&mv) {
                let count = if depth > 1 {
                    Self::perft(&new_pos, depth - 1)
                } else {
                    1
                };
                
                println!("{:?}: {}", mv, count);
                total += count;
            }
        }

        println!("Total: {}", total);
        total
    }

    /// Run standard perft tests from starting position
    pub fn run_standard_tests() {
        println!("Running standard perft tests from starting position...");
        
        let position = Position::startpos();
        
        // Expected results from standard perft tests
        let expected_results = [
            (1, 20),
            (2, 400),
            (3, 8902),
            (4, 197281),
            // (5, 4865609), // Commented out for faster testing
        ];

        for (depth, expected) in expected_results {
            let start = std::time::Instant::now();
            let result = Self::perft(&position, depth);
            let elapsed = start.elapsed();
            
            let status = if result == expected { "PASS" } else { "FAIL" };
            println!("Perft({}) = {} [{}] (expected: {}) in {:?}", 
                     depth, result, status, expected, elapsed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::types::Square;

    #[test]
    fn test_perft_depth_1() {
        let position = Position::startpos();
        assert_eq!(PerftTest::perft(&position, 1), 20);
    }

    #[test]
    fn test_perft_depth_2() {
        let position = Position::startpos();
        assert_eq!(PerftTest::perft(&position, 2), 400);
    }

    #[test]
    fn test_perft_depth_3() {
        let position = Position::startpos();
        assert_eq!(PerftTest::perft(&position, 3), 8902);
    }
    
    #[test]
    fn debug_perft_divide() {
        let position = Position::startpos();
        
        println!("\nRunning perft divide for depth 3:");
        let result = PerftTest::perft_divide(&position, 3);
        println!("Total: {}", result);
        
        // Expected: should be 8902, but we're getting 8888
        // This means we're missing 14 moves
    }
    
    #[test]
    fn debug_move_generation_issues() {
        // Test if we're generating duplicate moves or illegal moves
        let pos = Position::startpos();
        
        // Test depth 1 - should be exactly 20
        let moves_d1 = pos.legal_moves();
        println!("Depth 1: {} moves", moves_d1.len());
        assert_eq!(moves_d1.len(), 20);
        
        // Test depth 2 manually by playing each move and counting
        let mut total_d2 = 0;
        for mv in &moves_d1 {
            if let Some(new_pos) = pos.play(mv) {
                let moves_after = new_pos.legal_moves();
                total_d2 += moves_after.len();
                if mv.from == Square::E2 && mv.to == Square::E4 {
                    println!("After 1.e4: {} legal moves", moves_after.len());
                    // Should be 20 legal moves for black after 1.e4
                }
            }
        }
        println!("Depth 2 manual count: {}", total_d2);
        
        // Compare with perft
        let perft_d2 = PerftTest::perft(&pos, 2);
        println!("Depth 2 perft: {}", perft_d2);
        
        if total_d2 != perft_d2 as usize {
            println!("MISMATCH: Manual count {} vs perft {}", total_d2, perft_d2);
        }
        
        // Test if we can find duplicate moves
        let mut move_strings = Vec::new();
        for mv in &moves_d1 {
            let move_str = format!("{:?}", mv);
            if move_strings.contains(&move_str) {
                println!("DUPLICATE MOVE FOUND: {}", move_str);
            }
            move_strings.push(move_str);
        }
    }
}