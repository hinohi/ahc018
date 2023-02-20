use ahc018::{
    dsu::Dsu,
    judge::{DigResult, ExternalJudge, Judge},
    predict_h::gen_h,
    Grid, Point, N,
};
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use std::io::{stdin, BufRead};

pub struct Solver {
    grid: Grid<bool>,
    water: Vec<Point>,
    house: Vec<Point>,
}

struct Node {
    p: Point,
    is_water: bool,
}

struct Edge {
    i: usize,
    j: usize,
    l: u32,
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
        let mut nodes = Vec::with_capacity(self.house.len() + self.water.len());
        for &p in self.house.iter() {
            nodes.push(Node { p, is_water: false });
        }
        for &p in self.water.iter() {
            nodes.push(Node { p, is_water: true });
        }
        let mut water_id = Vec::with_capacity(self.water.len());
        let mut edges = Vec::with_capacity(nodes.len() * (nodes.len() - 1) / 2);
        for (i, n_i) in nodes.iter().enumerate() {
            if n_i.is_water {
                water_id.push(i);
            }
            for (j, n_j) in nodes.iter().enumerate().skip(i) {
                if n_i.is_water && n_j.is_water {
                    continue;
                }
                edges.push(Edge {
                    i,
                    j,
                    l: n_i.p.manhattan(&n_j.p),
                });
            }
        }
        edges.sort_by(|a, b| a.l.cmp(&b.l));

        let mut connects = Vec::new();
        let mut uf = Dsu::new(nodes.len());
        for edge in edges {
            if uf.same(edge.i, edge.j) {
                continue;
            }
            if water_id.iter().any(|&i| uf.same(i, edge.i))
                && water_id.iter().any(|&i| uf.same(i, edge.j))
            {
                continue;
            }
            uf.merge(edge.i, edge.j);
            connects.push(edge);
        }

        let mut ans = Vec::new();
        for edge in connects {
            let start = nodes[edge.i].p;
            let target = nodes[edge.j].p;
            self.restore_path(start, target, &mut ans, rng);
        }
        ans
    }

    fn restore_path(
        &mut self,
        start: Point,
        target: Point,
        path: &mut Vec<Point>,
        rng: &mut Mcg128Xsl64,
    ) {
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

        let update = |p, grid: &mut Grid<bool>, path: &mut Vec<Point>| {
            if !grid[p] {
                grid[p] = true;
                path.push(p);
            }
        };

        let go_x = |mut p: Point, grid: &mut Grid<bool>, path: &mut Vec<Point>| {
            while p.x() != target.x() {
                p = Point::new(p.x().wrapping_add(dx), p.y());
                update(p, grid, &mut *path);
            }
            p
        };
        let go_y = |mut p: Point, grid: &mut Grid<bool>, path: &mut Vec<Point>| {
            while p.y() != target.y() {
                p = Point::new(p.x(), p.y().wrapping_add(dy));
                update(p, grid, &mut *path);
            }
            p
        };

        let mut p = start;
        update(p, &mut self.grid, path);
        if rng.gen_bool(0.5) {
            p = go_x(p, &mut self.grid, path);
            go_y(p, &mut self.grid, path);
        } else {
            p = go_y(p, &mut self.grid, path);
            go_x(p, &mut self.grid, path);
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

        let parse_v = |stdin: &mut R, n: usize| {
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                let mut words = buf.split_whitespace();
                v.push((
                    words.next().unwrap().parse().unwrap(),
                    words.next().unwrap().parse().unwrap(),
                ));
            }
            v
        };
        let water = parse_v(stdin, w);
        let house = parse_v(stdin, k);
        Input { n, c, water, house }
    }
}

fn main() {
    let mut stdin = std::io::BufReader::new(stdin());
    let input = Input::new(&mut stdin);
    assert_eq!(input.n, N);

    let mut rng = Mcg128Xsl64::new(1);

    let mut landmark = Vec::new();
    for &(x, y) in input.water.iter() {
        landmark.push((x as usize, y as usize));
    }
    for &(x, y) in input.house.iter() {
        landmark.push((x as usize, y as usize));
    }
    for _ in 0..10 {
        let h = gen_h(&mut rng, &landmark, 30.0);
    }
    let ans = {
        let mut best = Vec::new();
        for _ in 0..100 {
            let mut solver = Solver::new(&input.water, &input.house);
            let ans = solver.solve(&mut rng);
            if best.is_empty() || best.len() > ans.len() {
                best = ans;
            }
        }
        best
    };

    let p = match input.c {
        1 => 37,
        2 => 51,
        4 => 71,
        8 => 100,
        16 => 138,
        32 => 192,
        64 => 264,
        128 => 363,
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
