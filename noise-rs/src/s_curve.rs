/// Quintic Interpolation Trait
///
/// Interpolates the provided value according to the quintic S-curve function
/// 6x<sup>5</sup> - 15x<sup>4</sup> + 10x<sup>3</sup>. This creates a curve with endpoints (0,0)
/// and (1,1), and first and second derivatives of zero at the endpoints, allowing the curves to be
/// combined together without discontinuities.
///
/// Values outside the range of [0, 1] will be clamped to the range before mapping.
pub trait Quintic {
    fn map_quintic(&self) -> Self;
}

impl Quintic for f64 {
    fn map_quintic(&self) -> Self {
        let x = self.clamp(0.0, 1.0);

        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    }
}
