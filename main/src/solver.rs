use crate::{predict_h::gen_h, Grid, Point, SetMinMax, N};
use rand::seq::SliceRandom;
use rand_pcg::Mcg128Xsl64;
use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solver {
    grid: Grid<bool>,
    water: Vec<Point>,
    house: Vec<Point>,
    c: u32,
    h: Vec<Grid<u32>>,
    guess_cost: Grid<u32>,
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

fn guess_power(h: &[Grid<u32>], p: Point, c: u32, s: u32) -> (u32, u32) {
    let v = h
        .iter()
        .filter_map(|h| if s < h[p] { Some(h[p] - s) } else { None })
        .collect::<Vec<_>>();
    if v.is_empty() {
        return (100, 100 + c);
    }
    if v.len() == 1 {
        return (v[0], v[0] + c);
    }
    let mut best = std::u32::MAX;
    let mut best_power = 100;
    for q in 10..=5000 {
        let cost = v
            .iter()
            .map(|&d| ((c + q) * ((d + q - 1) / q)))
            .sum::<u32>();
        if best.setmin(cost) {
            best_power = q;
        } else {
            return (best_power, best);
        }
    }
    (best_power, best)
}

impl Solver {
    pub fn new(
        rng: &mut Mcg128Xsl64,
        water: &[(u32, u32)],
        house: &[(u32, u32)],
        c: u32,
    ) -> Solver {
        let mut landmark = Vec::new();
        for &(x, y) in water.iter() {
            landmark.push((x as usize, y as usize));
        }
        for &(x, y) in house.iter() {
            landmark.push((x as usize, y as usize));
        }
        let mut h = Vec::with_capacity(10);
        for _ in 0..10 {
            h.push(Grid::from_vec(gen_h(rng, &landmark, 30.0)));
        }
        let mut guess_cost = Grid::new(0);
        for x in 0..N as u32 {
            for y in 0..N as u32 {
                let p = Point::new(x, y);
                guess_cost[p] = guess_power(&h, p, c, 0).1;
            }
        }
        Solver {
            grid: Grid::new(false),
            water: water.iter().map(|&(x, y)| Point::new(x, y)).collect(),
            house: house.iter().map(|&(x, y)| Point::new(x, y)).collect(),
            c,
            h,
            guess_cost,
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
                let w = s.w + self.guess_cost[n] as u64;
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

    pub fn solve(&mut self, rng: &mut Mcg128Xsl64) -> (Vec<Point>, u64) {
        let mut house = self.house.clone();
        house.shuffle(rng);

        let mut ans = Vec::new();
        let mut cost = 0;
        for start in house {
            let (target, prev) = self.dijkstra(start);
            let mut cur = target;
            loop {
                if !self.grid[cur] {
                    self.grid[cur] = true;
                    ans.push(cur);
                    cost += self.guess_cost[cur] as u64;
                }
                if let Some(nex) = prev[cur] {
                    cur = nex;
                } else {
                    break;
                }
            }
        }
        (ans, cost)
    }

    pub fn guess_power(&self, p: Point, s: u32) -> (u32, u32) {
        guess_power(&self.h, p, self.c, s)
    }
}
