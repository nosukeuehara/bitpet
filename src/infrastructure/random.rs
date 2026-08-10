pub trait RandomSource {
    fn next_u32(&mut self) -> u32;
}
