pub struct Outcome(pub f32);

impl Outcome {
    pub fn from_str(str: &str) -> Self {
        match str {
            "1-0" => Self(1.0),
            "0-1" => Self(0.0),
            "1/2-1/2" => Self(0.5),
            _ => panic!("Invalid outcome: {}", str),
        }
    }
}

#[rustfmt::skip]
pub const OUTCOMES: [&str; 3] = ["1-0", "0-1", "1/2-1/2"];
