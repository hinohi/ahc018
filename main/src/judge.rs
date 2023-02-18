use std::io::{BufRead, Stdout, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DigResult {
    NotBreak,
    Break,
    BreakAndFinish,
    Error,
}

pub trait Judge {
    fn dig(&mut self, x: usize, y: usize, p: u32) -> DigResult;
}

pub struct ExternalJudge<R> {
    stdin: R,
    stdout: Stdout,
}

impl<R> ExternalJudge<R> {
    pub fn new(stdin: R) -> ExternalJudge<R> {
        ExternalJudge {
            stdin,
            stdout: std::io::stdout(),
        }
    }
}

impl<R: BufRead> Judge for ExternalJudge<R> {
    fn dig(&mut self, x: usize, y: usize, p: u32) -> DigResult {
        writeln!(self.stdout, "{} {} {}", x, y, p).unwrap();
        self.stdout.flush().unwrap();
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        match buf.trim() {
            "0" => DigResult::NotBreak,
            "1" => DigResult::Break,
            "2" => DigResult::BreakAndFinish,
            "-1" => DigResult::Error,
            _ => unreachable!(),
        }
    }
}
