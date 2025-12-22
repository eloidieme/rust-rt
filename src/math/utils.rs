use rand::{SeedableRng, prelude::*, rngs::SmallRng};
use std::cell::RefCell;

thread_local! {
    static ORACLE: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng())
}

/// Returns a random float in [0.0, 1.0)
pub fn random() -> f64 {
    ORACLE.with(|rng| rng.borrow_mut().random())
}

/// Returns a random float in [min, max)
pub fn random_range(min: f64, max: f64) -> f64 {
    ORACLE.with(|rng| rng.borrow_mut().random_range(min..max))
}
