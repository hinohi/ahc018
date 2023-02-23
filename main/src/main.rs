use ahc018::{
    judge::{DigResult, ExternalJudge, Judge},
    solver::Solver,
    N,
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

    let mut solver = Solver::new(&mut rng, &input.water, &input.house);
    let ans = {
        let mut best = Vec::new();
        for _ in 0..10 {
            solver.reset();
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
