pub struct Union {
    pub parent: Vec<i32>,
    pub counts: i32
}

impl Union {
    pub fn new(n: usize) -> Self {
        let mut union = Union {
            parent: vec![0; n],
            counts: n as i32
        };
        for i in 0..n {
            union.parent[i] = i as i32;
        }
        union
    }

    pub fn find(&mut self, i: i32) -> i32 {
        if self.parent[i as usize] == i {
            return i
        }else {
            self.parent[i as usize] = self.find(self.parent[i as usize]);
            return self.parent[i as usize]
        }
    }

    pub fn merge(&mut self, x: i32, y: i32) {
        let rootx = self.find(x);
        let rooty = self.find(y);
        if rootx == rooty {
            return
        }
        self.parent[rootx as usize] = rooty;
        self.counts -= 1;
    }
}

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let len = row.len();
        let mut union = Union::new(len / 2);
        for i in (0..len).step_by(2) {
            union.merge(row[i]/2, row[i+1]/2)
        }
        (len / 2) as i32 - union.counts

    }
}