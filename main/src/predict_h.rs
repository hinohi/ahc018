use noise::Perlin;
use rand::Rng;

use crate::N;

struct LandGen {
    perlin: Perlin,
    y_offset1: f64,
    x_offset1: f64,
    y_offset2: f64,
    x_offset2: f64,
    freq1: f64,
    freq2: f64,
    power: f64,
}

impl LandGen {
    fn new<R: Rng>(rng: &mut R) -> LandGen {
        LandGen {
            perlin: Perlin::new(rng.gen()),
            y_offset1: rng.gen::<f64>(),
            x_offset1: rng.gen::<f64>(),
            y_offset2: rng.gen::<f64>(),
            x_offset2: rng.gen::<f64>(),
            freq1: rng.gen_range(2.0..8.0),
            freq2: rng.gen_range(10.0..20.0),
            power: rng.gen_range(2.0..4.0),
        }
    }

    fn get(&self, r: usize, c: usize) -> f64 {
        let y1 = self.y_offset1 + (r as f64 / N as f64) * self.freq1;
        let x1 = self.x_offset1 + (c as f64 / N as f64) * self.freq1;
        let y2 = self.y_offset2 + (r as f64 / N as f64) * self.freq2;
        let x2 = self.x_offset2 + (c as f64 / N as f64) * self.freq2;
        let h = self.perlin.get([x1, y1]) + self.perlin.get([x2, y2]) * 0.2;
        let h = 1.0 / (1.0 + (-3.0 * (h - 0.25)).exp());
        h.powf(self.power)
    }
}

pub fn gen_h<R: Rng>(rng: &mut R, landmark: &[(usize, usize)], th: f64) -> Vec<u32> {
    let th = 0.8 / 5000.0 * th;
    loop {
        let get = LandGen::new(rng);
        if landmark.iter().any(|&(r, c)| get.get(r, c) > th) {
            continue;
        }
        let mut h = Vec::with_capacity(N * N);
        let mut min = std::f64::MAX;
        let mut max = std::f64::MIN;
        for r in 0..N {
            for c in 0..N {
                let v = get.get(r, c);
                min = min.min(v);
                max = max.max(v);
                h.push(v);
            }
        }
        break h
            .into_iter()
            .map(|h| {
                let h = (10.0 + (h - min) / (max - min) * (5000.0 - 10.0)).round();
                h as u32
            })
            .collect();
    }
}
