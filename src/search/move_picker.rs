use shakmaty::{Move, Role};

pub struct MovePicker {
    scored_moves: Vec<(i32, Move)>,
}

impl MovePicker {
    pub(crate) fn new(moves: Vec<Move>) -> Self {
        let mut scored_moves: Vec<(i32, Move)> = moves
            .into_iter()
            .map(|m| (Self::score_move(&m), m))
            .collect();
        scored_moves.sort_unstable_by_key(|(s, _)| *s);

        Self { scored_moves }
    }

    pub(crate) fn next(&mut self) -> Option<Move> {
        self.scored_moves.pop().map(|(_, m)| m)
    }

    fn score_move(m: &Move) -> i32 {
        let mut score = 0;

        if m.is_promotion() {
            score += match m.promotion() {
                Some(Role::Queen) => 9,
                Some(Role::Rook) => 5,
                Some(Role::Bishop) => 3,
                Some(Role::Knight) => 3,
                _ => 0,
            };
        }

        if m.is_capture() {
            let role_value = match m.role() {
                Role::Pawn => 1,
                Role::Knight => 3,
                Role::Bishop => 3,
                Role::Rook => 5,
                Role::Queen => 9,
                Role::King => 0,
            };

            let cap_value = m
                .capture()
                .map(|c| match c {
                    Role::Pawn => 1,
                    Role::Knight => 3,
                    Role::Bishop => 3,
                    Role::Rook => 5,
                    Role::Queen => 9,
                    Role::King => 0,
                })
                .unwrap_or(0);

            score += cap_value - role_value
        }

        score
    }
}
