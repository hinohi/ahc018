use ahc018::{
    abs_diff,
    judge::{DigResult, ExternalJudge, Judge},
    Grid, Point, N,
};
use rand::seq::SliceRandom;
use rand_pcg::Mcg128Xsl64;
use std::io::{stdin, BufRead};

pub struct Solver {
    grid: Grid<bool>,
    water: Vec<Point>,
    house: Vec<Point>,
}

impl Solver {
    pub fn new(water: &[(u32, u32)], house: &[(u32, u32)]) -> Solver {
        Solver {
            grid: Grid::new(false),
            water: water.iter().map(|&(x, y)| Point::new(x, y)).collect(),
            house: house.iter().map(|&(x, y)| Point::new(x, y)).collect(),
        }
    }

    pub fn solve(&mut self, rng: &mut Mcg128Xsl64) -> Vec<Point> {
        let mut ans = Vec::new();
        let mut house = self.house.clone();
        house.shuffle(rng);
        for &start in house.iter() {
            let target = self.bfs(start);
            self.restore_path(start, target, &mut ans);
        }
        ans
    }

    fn bfs(&self, start: Point) -> Point {
        let mut grid = self.grid.clone();
        let mut queue = std::collections::VecDeque::new();
        queue.push_front(start);
        while let Some(p) = queue.pop_front() {
            grid[p] = true;
            if self.water.iter().any(|&q| p == q) {
                return p;
            }
            for nei in p.neighbors() {
                // 既存の堀跡と繋がったらそこで終わり
                if self.grid[nei] {
                    return nei;
                }
                // 未採掘
                if !grid[nei] {
                    grid[nei] = true;
                    queue.push_back(nei);
                }
            }
        }
        unreachable!()
    }

    fn restore_path(&mut self, start: Point, target: Point, path: &mut Vec<Point>) {
        use std::cmp::Ordering;
        let dx = match start.x().cmp(&target.x()) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => !0,
        };
        let dy = match start.y().cmp(&target.y()) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => !0,
        };
        let rx = abs_diff(start.y(), target.y()) + 1;
        let ry = abs_diff(start.x(), target.x()) + 1;
        let mut p = start;
        if !self.grid[p] {
            self.grid[p] = true;
            path.push(p);
        }
        while p.x() != target.x() || p.y() != target.y() {
            if abs_diff(p.x(), target.x()) * rx > abs_diff(p.y(), target.y()) * ry {
                p = Point::new(p.x().wrapping_add(dx), p.y());
            } else {
                p = Point::new(p.x(), p.y().wrapping_add(dy));
            }
            if !self.grid[p] {
                self.grid[p] = true;
                path.push(p);
            }
        }
    }
}

struct Input {
    n: usize,
    c: u32,
    water: Vec<(u32, u32)>,
    house: Vec<(u32, u32)>,
}

impl Input {
    fn new<R: BufRead>(stdin: &mut R) -> Input {
        let (n, w, k, c) = {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            let mut words = buf.split_whitespace();
            (
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
            )
        };

        let mut water = Vec::with_capacity(w);
        for _ in 0..w {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            let mut words = buf.split_whitespace();
            water.push((
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
            ));
        }
        let mut house = Vec::with_capacity(k);
        for _ in 0..k {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            let mut words = buf.split_whitespace();
            house.push((
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
            ));
        }
        Input { n, c, water, house }
    }
}

fn main() {
    let mut stdin = std::io::BufReader::new(stdin());
    let input = Input::new(&mut stdin);
    assert_eq!(input.n, N);

    let mut rng = Mcg128Xsl64::new(1);
    let ans = {
        let mut best = Vec::new();
        for _ in 0..10 {
            let mut solver = Solver::new(&input.water, &input.house);
            let ans = solver.solve(&mut rng);
            if best.is_empty() || best.len() > ans.len() {
                best = ans;
            }
        }
        best
    };

    let p = match input.c {
        1 => 71,
        2 => 91,
        4 => 119,
        8 => 162,
        16 => 218,
        32 => 295,
        64 => 417,
        128 => 556,
        _ => unreachable!(),
    };

    let mut judge = ExternalJudge::new(stdin);
    for a in ans {
        loop {
            let r = judge.dig(a.x() as usize, a.y() as usize, p);
            // eprintln!("{:?} {:?}", p, r);
            match r {
                DigResult::NotBreak => continue,
                DigResult::Break => break,
                DigResult::BreakAndFinish | DigResult::Error => return,
            }
        }
    }
    eprintln!("答えが足らん");
}
