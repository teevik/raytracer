#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const fn new(start: f32, end: f32) -> Self {
        Self {
            min: start,
            max: end,
        }
    }

    pub fn combine(a: Self, b: Self) -> Self {
        Self::new(f32::min(a.min, b.min), f32::max(a.max, b.max))
    }

    pub fn contains(self, value: f32) -> bool {
        value >= self.min && value < self.max
    }

    pub fn expand(self, delta: f32) -> Self {
        let padding = delta / 2.;

        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}
