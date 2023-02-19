use core::ops::Sub;
use num_traits::MulAdd;

/// Performs linear interpolation between two values.
#[cfg(not(target_os = "emscripten"))]
#[inline]
pub(crate) fn linear<T>(a: T, b: T, x: T) -> T
where
    T: MulAdd<Output = T> + Sub<Output = T> + Copy,
{
    x.mul_add(b - a, a)
}
