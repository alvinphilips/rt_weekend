pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI_FRAC_180: f64 = PI / 180.0;
pub const NEAR_ZERO: f64 = 1e-15;

/// Convert from degrees to radian.
///
/// The value of `degrees` is multiplied by [`PI_FRAC_180`].
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI_FRAC_180
}

pub fn clamp<T: std::cmp::PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}
