
#[cfg(feature = "datagen")]
pub struct Rng {
    state: u64,
}

#[cfg(feature = "datagen")]
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 16) as u32
    }

    pub fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        let span = range.end - range.start;
        range.start + (self.next_u32() as usize % span)
    }
}
