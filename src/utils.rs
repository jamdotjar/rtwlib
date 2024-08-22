use core::ops::Range;

pub trait RangeExtensions {
    fn size(&self) -> f64;
    fn contains(&self, n: f64) -> bool;
    fn surrounds(&self, n: f64) -> bool;
}

impl RangeExtensions for Range<f64> {
    fn size(&self) -> f64 {
        return self.end - self.start;
    }
    fn contains(&self, n: f64) -> bool {
        return self.start <= n && n <= self.end;
    }
    fn surrounds(&self, n: f64) -> bool {
        return self.start < n && n <= self.end;
    }
}


