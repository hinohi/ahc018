use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng, SeedableRng,
};
use rand_xorshift::XorShiftRng;

const TABLE_SIZE: usize = 256;

pub trait NoiseHasher: Send + Sync {
    fn hash(&self, to_hash: &[isize]) -> usize;
}

/// A seed table, required by all noise functions.
///
/// Table creation is expensive, so in most circumstances you'll only want to
/// create one of these per generator.
#[derive(Copy, Clone)]
pub struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl Distribution<PermutationTable> for Standard {
    /// Generates a PermutationTable using a random seed.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PermutationTable {
        let mut perm_table = PermutationTable {
            values: [0; TABLE_SIZE],
        };

        perm_table
            .values
            .iter_mut()
            .enumerate()
            .for_each(|(i, b)| *b = i as u8);
        perm_table.values.shuffle(rng);

        perm_table
    }
}

impl PermutationTable {
    /// Deterministically generates a new permutation table based on a `u32` seed value.
    ///
    /// Internally this uses a `XorShiftRng`, but we don't really need to worry
    /// about cryptographic security when working with procedural noise.
    pub fn new(seed: u32) -> Self {
        let mut real = [0; 16];
        real[0] = 1;
        for i in 1..4 {
            real[i * 4] = seed as u8;
            real[(i * 4) + 1] = (seed >> 8) as u8;
            real[(i * 4) + 2] = (seed >> 16) as u8;
            real[(i * 4) + 3] = (seed >> 24) as u8;
        }
        let mut rng: XorShiftRng = SeedableRng::from_seed(real);
        rng.gen()
    }
}

impl NoiseHasher for PermutationTable {
    fn hash(&self, to_hash: &[isize]) -> usize {
        let index = to_hash
            .iter()
            .map(|&a| (a & 0xff) as usize)
            .reduce(|a, b| self.values[a] as usize ^ b)
            .unwrap();
        self.values[index] as usize
    }
}
