use crate::chess::{Position, MoveGenerator};

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
}