#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
/// Represents a real-valued interval [min, max].
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
    /// An empty interval.
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    /// An interval covering the entire real line.
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// Creates a new interval [min, max].
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    /// Returns the size (length) of the interval.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if the interval contains x (inclusive).
    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    /// Checks if the interval strictly surrounds x (exclusive).
    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }

    /// Clamps x to the interval [min, max].
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    /// Expands the interval by delta on both sides.
    pub fn expand(&self, delta: f64) -> Interval {
        Interval::new(self.min - delta, self.max + delta)
    }

    /// Returns the smallest interval containing both a and b.
    pub fn merge(a: Interval, b: Interval) -> Interval {
        Interval::new(f64::min(a.min, b.min), f64::max(a.max, b.max))
    }
}
