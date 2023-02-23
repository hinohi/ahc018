use ahc018::{
    judge::{DigResult, ExternalJudge, Judge},
    solver::Solver,
    SetMinMax, N,
};
use rand_pcg::Mcg128Xsl64;
use std::io::{stdin, BufRead};

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

    let mut solver = Solver::new(&mut rng, &input.water, &input.house, input.c);
    let ans = {
        let mut best = Vec::new();
        let mut best_cost = std::u64::MAX;
        for _ in 0..10 {
            solver.reset();
            let (ans, cost) = solver.solve(&mut rng);
            if best_cost.setmin(cost) {
                best = ans;
            }
        }
        best
    };

    let mut judge = ExternalJudge::new(stdin);
    for p in ans {
        let mut s = 0;
        loop {
            let (power, cost) = solver.guess_power(p, s);
            println!("# {} {}", power, cost);
            let r = judge.dig(p.x() as usize, p.y() as usize, power);
            s += power;
            match r {
                DigResult::NotBreak => continue,
                DigResult::Break => break,
                DigResult::BreakAndFinish | DigResult::Error => return,
            }
        }
    }
    eprintln!("答えが足らん");
}
