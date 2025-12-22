#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        Interval::new(self.min - delta, self.max + delta)
    }

    pub fn merge(a: Interval, b: Interval) -> Interval {
        Interval::new(f64::min(a.min, b.min), f64::max(a.max, b.max))
    }
}
