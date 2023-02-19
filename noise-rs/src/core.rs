use crate::{permutationtable::NoiseHasher, s_curve::Quintic};
use core::f64;

#[inline(always)]
pub fn perlin_2d<NH>(point: [f64; 2], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    //
    // 1/(sqrt(N)/2), N=2 -> sqrt(2)
    const SCALE_FACTOR: f64 = f64::consts::SQRT_2;

    let [x, y] = point;

    #[inline(always)]
    #[rustfmt::skip]
    fn gradient_dot_v(perm: usize, x: f64, y: f64) -> f64 {
        match perm & 0b11 {
            0 =>  x + y, // ( 1,  1)
            1 => -x + y, // (-1,  1)
            2 =>  x - y, // ( 1, -1)
            3 => -x - y, // (-1, -1)
            _ => unreachable!(),
        }
    }

    let [fx, fy] = [x.floor(), y.floor()];
    let [cx, cy] = [fx as isize, fy as isize];
    let [dx, dy] = [x - fx, y - fy];

    macro_rules! call_gradient(
        ($x:expr, $y:expr) => {
            {
                gradient_dot_v(
                    hasher.hash(&[cx + $x, cy + $y]),
                    dx - $x as f64,
                    dy - $y as f64,
                )
            }
        }
    );

    let g00 = call_gradient!(0, 0);
    let g10 = call_gradient!(1, 0);
    let g01 = call_gradient!(0, 1);
    let g11 = call_gradient!(1, 1);

    let u = dx.map_quintic();
    let v = dy.map_quintic();

    let unscaled_result = bilinear_interpolation(u, v, g00, g01, g10, g11);

    let scaled_result = unscaled_result * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    scaled_result.clamp(-1.0, 1.0)
}

#[inline(always)]
fn bilinear_interpolation(u: f64, v: f64, g00: f64, g01: f64, g10: f64, g11: f64) -> f64 {
    let k0 = g00;
    let k1 = g10 - g00;
    let k2 = g01 - g00;
    let k3 = g00 + g11 - g10 - g01;

    k0 + k1 * u + k2 * v + k3 * u * v
}
