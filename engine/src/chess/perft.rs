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
            let new_pos = position.play_unchecked(&mv);
            count += Self::perft(&new_pos, depth - 1);
        }

        count
    }

    /// Perft with move breakdown for debugging
    pub fn perft_divide(position: &Position, depth: u8) -> u64 {
        let moves = position.legal_moves();
        let mut total = 0;

        for mv in moves {
            let new_pos = position.play_unchecked(&mv);
            let count = if depth > 1 {
                Self::perft(&new_pos, depth - 1)
            } else {
                1
            };
            
            println!("{:?}: {}", mv, count);
            total += count;
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
            (5, 4865609),
            (6, 119060324),
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

    /// Run perft tests on specific positions mentioned in the issue
    pub fn run_challenge_tests() {
        println!("Running challenge perft tests...");
        
        // Test positions from the comment
        let test_positions = [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "Starting position",
                119060324
            ),
            (
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
                "Kiwipete position",
                8031647685
            ),
            (
                "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
                "Endgame position",
                11030083
            ),
            (
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                "Complex position",
                706045033
            ),
        ];

        for (fen, description, expected_depth_6) in test_positions {
            println!("\n=== {} ===", description);
            println!("FEN: {}", fen);
            
            match Position::from_fen(fen) {
                Ok(position) => {
                    // Test progressively deeper depths
                    for depth in 1..=6 {
                        let start = std::time::Instant::now();
                        let result = Self::perft(&position, depth);
                        let elapsed = start.elapsed();
                        
                        let expected = if depth == 6 { Some(expected_depth_6) } else { None };
                        let status = if let Some(exp) = expected {
                            if result == exp { "PASS" } else { "FAIL" }
                        } else {
                            "INFO"
                        };
                        
                        println!("  Perft({}) = {} [{}]{} in {:?}", 
                                depth, result, status, 
                                if let Some(exp) = expected { format!(" (expected: {})", exp) } else { String::new() },
                                elapsed);
                        
                        // Stop if we take too long or fail
                        if elapsed.as_secs() > 60 {
                            println!("  Stopping due to timeout");
                            break;
                        }
                        if let Some(exp) = expected {
                            if result != exp {
                                println!("  Stopping due to failure");
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("  ERROR: Failed to parse FEN: {:?}", e);
                }
            }
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
    fn test_perft_depth_4() {
        let position = Position::startpos();
        assert_eq!(PerftTest::perft(&position, 4), 197281);
    }

    #[test]
    fn test_perft_depth_5() {
        let position = Position::startpos();
        assert_eq!(PerftTest::perft(&position, 5), 4865609);
    }

    #[test]
    fn test_perft_depth_6() {
        let position = Position::startpos();
        assert_eq!(PerftTest::perft(&position, 6), 119060324);
    }

    #[test]
    fn test_kiwipete_position() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
        let position = Position::from_fen(fen).expect("Valid FEN");
        
        // Test at lower depths first
        let expected_results = [
            (1, 48),
            (2, 2039),
            (3, 97862),
            (4, 4085603),
            (5, 193690690),
            // (6, 8031647685), // This is the target
        ];
        
        for (depth, expected) in expected_results {
            let result = PerftTest::perft(&position, depth);
            assert_eq!(result, expected, "Kiwipete perft({}) failed", depth);
        }
    }

    #[test]
    fn test_endgame_position() {
        let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        let position = Position::from_fen(fen).expect("Valid FEN");
        
        // Test at lower depths first
        let expected_results = [
            (1, 14),
            (2, 191),
            (3, 2812),
            (4, 43238),
            (5, 674624),
            // (6, 11030083), // This is the target
        ];
        
        for (depth, expected) in expected_results {
            let result = PerftTest::perft(&position, depth);
            assert_eq!(result, expected, "Endgame perft({}) failed", depth);
        }
    }

    #[test]
    fn test_complex_position() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let position = Position::from_fen(fen).expect("Valid FEN");
        
        // Test at lower depths first
        let expected_results = [
            (1, 6),
            (2, 264),
            (3, 9467),
            (4, 422333),
            (5, 15833292),
            // (6, 706045033), // This is the target
        ];
        
        for (depth, expected) in expected_results {
            let result = PerftTest::perft(&position, depth);
            assert_eq!(result, expected, "Complex perft({}) failed", depth);
        }
    }

    #[test]
    fn run_all_challenge_tests() {
        // This test will run all challenge positions
        PerftTest::run_challenge_tests();
    }
    
    #[test]
    fn debug_depth_4_issue() {
        let position = Position::startpos();
        
        // Let's test depth 4 manually
        println!("Testing depth 4 issue...");
        
        // Run perft divide at depth 4
        let result = PerftTest::perft_divide(&position, 4);
        
        // Expected is 197281, but we're getting 185438
        println!("Result: {}, Expected: 197281", result);
        println!("Difference: {}", (197281i64 - result as i64).abs());
        
        // The issue might be in our move generation or legal move checking
        // Let's verify some specific moves
        let moves = position.legal_moves();
        println!("Legal moves from start: {}", moves.len());
        
        // Check a few specific moves
        for (i, mv) in moves.iter().enumerate().take(3) {
            if let Some(new_pos) = position.play(mv) {
                let depth_3_result = PerftTest::perft(&new_pos, 3);
                println!("Move {}: {:?}, perft(3) = {}", i+1, mv, depth_3_result);
            }
        }
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
        
        // Test depth 3
        let perft_d3 = PerftTest::perft(&pos, 3);
        println!("Depth 3 perft: {}", perft_d3);
        
        // Test depth 4
        let perft_d4 = PerftTest::perft(&pos, 4);
        println!("Depth 4 perft: {} (expected: 197281)", perft_d4);
    }
}