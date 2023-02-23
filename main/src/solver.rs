use crate::{predict_h::gen_h, Grid, Point, SetMinMax};
use rand::seq::SliceRandom;
use rand_pcg::Mcg128Xsl64;
use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solver {
    grid: Grid<bool>,
    water: Vec<Point>,
    house: Vec<Point>,
    guess_height: Grid<u32>,
}

struct DState {
    p: Point,
    w: u64,
}

impl PartialEq for DState {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl Eq for DState {}

impl PartialOrd for DState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for DState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.w.cmp(&self.w)
    }
}

impl Solver {
    pub fn new(rng: &mut Mcg128Xsl64, water: &[(u32, u32)], house: &[(u32, u32)]) -> Solver {
        let mut landmark = Vec::new();
        for &(x, y) in water.iter() {
            landmark.push((x as usize, y as usize));
        }
        for &(x, y) in house.iter() {
            landmark.push((x as usize, y as usize));
        }
        let mut h = Grid::new(0);
        for _ in 0..10 {
            let g = Grid::from_vec(gen_h(rng, &landmark, 30.0));
            h.zip_map(&g, |h, g| *h += *g);
        }
        Solver {
            grid: Grid::new(false),
            water: water.iter().map(|&(x, y)| Point::new(x, y)).collect(),
            house: house.iter().map(|&(x, y)| Point::new(x, y)).collect(),
            guess_height: h,
        }
    }

    pub fn reset(&mut self) {
        self.grid = Grid::new(false);
    }

    fn dijkstra(&self, start: Point) -> (Point, Grid<Option<Point>>) {
        let mut heap = BinaryHeap::new();
        let mut dist = Grid::new(std::u64::MAX);
        let mut prev = Grid::new(None);
        dist[start] = 0;
        heap.push(DState { p: start, w: 0 });
        while let Some(s) = heap.pop() {
            if dist[s.p] < s.w {
                continue;
            }
            for n in s.p.neighbors() {
                if self.grid[n] {
                    prev[n] = Some(s.p);
                    return (n, prev);
                }
                let w = s.w + self.guess_height[n] as u64;
                if dist[n].setmin(w) {
                    prev[n] = Some(s.p);
                    if self.water.iter().any(|p| *p == n) {
                        return (n, prev);
                    }
                    heap.push(DState { p: n, w });
                }
            }
        }
        unreachable!()
    }

    pub fn solve(&mut self, rng: &mut Mcg128Xsl64) -> Vec<Point> {
        let mut house = self.house.clone();
        house.shuffle(rng);

        let mut ans = Vec::new();
        for start in house {
            let (target, prev) = self.dijkstra(start);
            let mut cur = target;
            loop {
                if !self.grid[cur] {
                    self.grid[cur] = true;
                    ans.push(cur);
                }
                if let Some(nex) = prev[cur] {
                    cur = nex;
                } else {
                    break;
                }
            }
        }
        ans
    }
}
