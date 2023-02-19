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

pub struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl Distribution<PermutationTable> for Standard {
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

pub trait Quintic {
    fn map_quintic(&self) -> Self;
}

impl Quintic for f64 {
    fn map_quintic(&self) -> Self {
        let x = self.clamp(0.0, 1.0);

        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    }
}

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
    const SCALE_FACTOR: f64 = std::f64::consts::SQRT_2;

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
