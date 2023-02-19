#[derive(Clone)]
pub struct Dsu {
    parent_or_size: Vec<i32>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        let (x, y) = (self.leader(a), self.leader(b));
        if x == y {
            return false;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            self.parent_or_size[y] += self.parent_or_size[x];
            self.parent_or_size[x] = y as i32;
        } else {
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
        }
        true
    }

    pub fn leader(&mut self, a: usize) -> usize {
        // let x = unsafe { *self.parent_or_size.get_unchecked(a) };
        let x = self.parent_or_size[a];
        if x < 0 {
            return a;
        }
        let x = self.leader(x as usize) as i32;
        self.parent_or_size[a] = x;
        x as usize
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }
}
