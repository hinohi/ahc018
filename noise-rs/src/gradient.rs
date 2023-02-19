#[inline(always)]
#[rustfmt::skip]
pub(crate) fn grad2(index: usize) -> [f64; 2] {
    // Vectors are combinations of -1, 0, and 1
    // Precompute the normalized element
    const DIAG : f64 = core::f64::consts::FRAC_1_SQRT_2;

    match index % 8 {
        0 => [  1.0,   0.0],
        1 => [ -1.0,   0.0],
        2 => [  0.0,   1.0],
        3 => [  0.0,  -1.0],
        4 => [ DIAG,  DIAG],
        5 => [-DIAG,  DIAG],
        6 => [ DIAG, -DIAG],
        7 => [-DIAG, -DIAG],
        _ => panic!("Attempt to access gradient {} of 8", index % 8),
    }
}
