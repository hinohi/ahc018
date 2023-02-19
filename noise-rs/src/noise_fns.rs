use alloc::boxed::Box;

use crate::{core::*, permutationtable::PermutationTable};

pub trait NoiseFn<T, const DIM: usize> {
    fn get(&self, point: [T; DIM]) -> f64;
}

impl<'a, T, M, const DIM: usize> NoiseFn<T, DIM> for &'a M
where
    M: NoiseFn<T, DIM> + ?Sized,
{
    #[inline]
    fn get(&self, point: [T; DIM]) -> f64 {
        M::get(*self, point)
    }
}

impl<T, M, const DIM: usize> NoiseFn<T, DIM> for Box<M>
where
    M: NoiseFn<T, DIM> + ?Sized,
{
    #[inline]
    fn get(&self, point: [T; DIM]) -> f64 {
        M::get(self, point)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Perlin {
    perm_table: PermutationTable,
}

impl Perlin {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(seed: u32) -> Self {
        Self {
            perm_table: PermutationTable::new(seed),
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

/// 2-dimensional perlin noise
impl NoiseFn<f64, 2> for Perlin {
    fn get(&self, point: [f64; 2]) -> f64 {
        perlin_2d(point, &self.perm_table)
    }
}
