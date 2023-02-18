use std::fs;

fn get_dist() -> Vec<i64> {
    let mut dist = vec![0; 5001];
    for entry in fs::read_dir("in").unwrap() {
        let path = entry.unwrap().path();
        let data = fs::read_to_string(path).unwrap();
        let mut tokens = data.split_whitespace();
        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();
        for _ in 0..200 * 200 {
            let n = tokens.next().unwrap().parse::<usize>().unwrap();
            dist[n] += 1;
        }
    }
    dist
}

fn calc_best_p(dist: &[i64], c: i64) -> i64 {
    let mut pre = i64::MAX;
    for p in 10.. {
        let cost = dist
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                if d == 0 {
                    0
                } else {
                    (c + p) * ((i as i64 + p - 1) / p)
                }
            })
            .sum::<i64>();
        if p % 10 == 0 {
            eprintln!("{} {} {}", c, p, cost);
        }
        if pre > cost {
            pre = cost;
        } else {
            return p - 1;
        }
    }
    unreachable!()
}

fn main() {
    let dist = get_dist();
    for c in [1, 2, 4, 8, 16, 32, 64, 128] {
        println!("{} {}", c, calc_best_p(&dist, c));
    }
}
