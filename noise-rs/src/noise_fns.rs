use crate::{core::*, permutationtable::PermutationTable};

#[derive(Clone, Copy)]
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

    pub fn get(&self, point: [f64; 2]) -> f64 {
        perlin_2d(point, &self.perm_table)
    }
}
