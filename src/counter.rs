#[derive(Debug, Clone)]
pub struct Counter {
    pub total_iecsrgb_profiles: usize,
    pub total_srgb_profiles: usize,
    pub total_no_emb_profiles: usize,
}
impl Counter {
    pub fn new() -> Self {
        Self {
            total_iecsrgb_profiles: 0,
            total_no_emb_profiles: 0,
            total_srgb_profiles: 0,
        }
    }
}
impl std::ops::Add for Counter {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.total_iecsrgb_profiles += rhs.total_iecsrgb_profiles;
        self.total_no_emb_profiles += rhs.total_no_emb_profiles;
        self.total_srgb_profiles += rhs.total_srgb_profiles;
        self
    }
}
