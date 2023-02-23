pub mod dsu;
mod grid;
pub mod judge;
pub mod predict_h;
pub mod solver;

pub use crate::grid::*;

pub const N: usize = 200;

pub fn abs_diff(x: u32, y: u32) -> u32 {
    if x < y {
        y - x
    } else {
        x - y
    }
}

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}

impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}
