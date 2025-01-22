//! A collection of miscelanious ( unused ) utility functions, might come in handy later.
use core::ops::Range;

//ignore doc tests
#[allow(missing_docs)]
pub trait RangeExtensions {
    fn size(&self) -> f64;
    fn contains(&self, n: f64) -> bool;
    fn surrounds(&self, n: f64) -> bool;
    fn clamp(&self, n: f64) -> f64;
}

impl RangeExtensions for Range<f64> {
    //Some fun utils for working wiutrh ranges (a..b)
    fn size(&self) -> f64 {
        return self.end - self.start;
    }
    fn contains(&self, n: f64) -> bool {
        return self.start <= n && n <= self.end;
    }
    fn surrounds(&self, n: f64) -> bool {
        return self.start < n && n <= self.end;
    }
    fn clamp(&self, n: f64) -> f64 {
        if n < self.start {
            return self.start;
        }
        if n > self.end {
            return self.start;
        }
        return n;
    }
}
