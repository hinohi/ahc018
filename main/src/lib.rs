mod grid;
pub mod judge;

pub use crate::grid::*;

pub const N: usize = 200;

pub fn abs_diff(x: u32, y: u32) -> u32 {
    if x < y {
        y - x
    } else {
        x - y
    }
}