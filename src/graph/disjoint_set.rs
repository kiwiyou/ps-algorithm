pub struct SetForest {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl SetForest {
    pub fn new(s: usize) -> Self {
        Self {
            parent: (0..s).collect(),
            rank: vec![1; s],
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        while i != self.parent[i] {
            // Path-halving
            self.parent[i] = self.parent[self.parent[i]];
            i = self.parent[i];
        }
        i
    }

    // Returns if two items were in the same set.
    pub fn join(&mut self, u: usize, v: usize) -> bool {
        let mut pu = self.find(u);
        let mut pv = self.find(v);
        if pu == pv {
            true
        } else {
            if self.rank[pu] < self.rank[pv] {
                std::mem::swap(&mut pu, &mut pv);
            }
            self.parent[pv] = pu;
            if self.rank[pu] == self.rank[pv] {
                self.rank[pu] += 1;
            }
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn join() {
        let mut forest = SetForest::new(5);
        assert_eq!(false, forest.join(1, 3));
        assert_eq!(vec![0, 1, 2, 1, 4], forest.parent);
        assert_eq!(vec![1, 2, 1, 1, 1], forest.rank);
        assert_eq!(false, forest.join(2, 4));
        assert_eq!(vec![0, 1, 2, 1, 2], forest.parent);
        assert_eq!(vec![1, 2, 2, 1, 1], forest.rank);
        assert_eq!(false, forest.join(0, 3));
        assert_eq!(vec![1, 1, 2, 1, 2], forest.parent);
        assert_eq!(vec![1, 2, 2, 1, 1], forest.rank);
        assert_eq!(true, forest.join(0, 1));
        assert_eq!(vec![1, 1, 2, 1, 2], forest.parent);
        assert_eq!(vec![1, 2, 2, 1, 1], forest.rank);
        assert_eq!(false, forest.join(1, 4));
        assert_eq!(vec![1, 1, 1, 1, 2], forest.parent);
        assert_eq!(vec![1, 3, 2, 1, 1], forest.rank);
    }

    #[test]
    fn find() {
        let mut forest = SetForest::new(5);
        assert_eq!(4, forest.find(4));
        forest.join(0, 1);
        assert_eq!(0, forest.find(1));
        forest.join(3, 2);
        assert_eq!(3, forest.find(2));
        forest.join(0, 2);
        assert_eq!(0, forest.find(3));
    }
}
