use crate::{abs_diff, N};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub fn x(self) -> u32 {
        self.x
    }

    pub fn y(self) -> u32 {
        self.y
    }

    pub fn manhattan(&self, other: &Point) -> u32 {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }

    pub fn neighbors(self) -> Vec<Point> {
        let mut nei = Vec::with_capacity(4);
        if self.x > 0 {
            nei.push(Point::new(self.x - 1, self.y));
        }
        if self.x + 1 < N as u32 {
            nei.push(Point::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            nei.push(Point::new(self.x, self.y - 1));
        }
        if self.y + 1 < N as u32 {
            nei.push(Point::new(self.x, self.y + 1));
        }
        nei
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(init: T) -> Grid<T> {
        Grid {
            data: vec![init; N * N],
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[(index.y * N as u32 + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[(index.y * N as u32 + index.x) as usize]
    }
}
