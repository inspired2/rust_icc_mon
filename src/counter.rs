#[derive(Debug, Clone)]
pub struct Counter {
    pub iec_srgb: usize,
    pub other_srgb: usize,
    pub no_profile: usize,
    pub adobe_rgb: usize,
    pub other: usize
}
impl Counter {
    pub fn new() -> Self {
        Self {
            iec_srgb: 0,
            no_profile: 0,
            other_srgb: 0,
            adobe_rgb: 0,
            other: 0
        }
    }
}
impl std::ops::Add for Counter {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.iec_srgb += rhs.iec_srgb;
        self.no_profile += rhs.no_profile;
        self.other_srgb += rhs.other_srgb;
        self.adobe_rgb += rhs.adobe_rgb;
        self.other += rhs.other;
        self
    }
}
