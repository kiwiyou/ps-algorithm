use super::Graph;

pub struct HeavyLight {
    parents: Vec<usize>,
    depths: Vec<usize>,
    chains: Vec<usize>,
    heads: Vec<usize>,
    pos: Vec<usize>,
}

impl HeavyLight {
    fn fill_chains<T>(&mut self, root: usize, graph: &Graph<T>) {
        self.depths[root] = 1;
        self.parents[root] = root;
        let mut stack = Vec::with_capacity(graph.node_count());
        let mut sizes = vec![(0, 0); graph.node_count()];
        stack.push(root);
        while let Some(&now) = stack.last() {
            let parent = self.parents[now];
            if sizes[now].0 == 0usize {
                sizes[now].0 += 1;
                for (next, _) in graph.neighbors(now) {
                    if next != parent {
                        self.depths[next] = self.depths[now] + 1;
                        self.parents[next] = now;
                        stack.push(next);
                    }
                }
            } else {
                if now != parent {
                    sizes[parent].0 += sizes[now].0;
                    if sizes[parent].1 < sizes[now].0 {
                        sizes[parent].1 = sizes[now].0;
                        self.chains[parent] = now;
                    }
                }
                stack.pop();
            }
        }
    }

    fn fill_heads<T>(&mut self, root: usize, graph: &Graph<T>) {
        let mut pos = 0;
        let mut stack = Vec::with_capacity(graph.node_count());
        stack.push((root, root));
        while let Some((now, head)) = stack.pop() {
            self.heads[now] = head;
            self.pos[now] = pos;
            pos += 1;
            let chain = self.chains[now];
            let parent = self.parents[now];
            for (next, _) in graph.neighbors(now) {
                if next != parent && next != chain {
                    stack.push((next, next));
                }
            }
            if chain != now {
                stack.push((chain, head));
            }
        }
    }

    pub fn new<T>(root: usize, graph: &Graph<T>) -> Self {
        let mut obj = Self {
            parents: vec![0; graph.node_count()],
            depths: vec![0; graph.node_count()],
            chains: (0..graph.node_count()).collect(),
            heads: vec![0; graph.node_count()],
            pos: vec![0; graph.node_count()],
        };
        obj.fill_chains(root, graph);
        obj.fill_heads(root, graph);
        obj
    }

    pub fn path(&self, u: usize, v: usize) -> Path {
        Path {
            hld: self,
            endpoints: Some((u, v)),
        }
    }
}

pub struct Path<'a> {
    hld: &'a HeavyLight,
    endpoints: Option<(usize, usize)>,
}

impl Iterator for Path<'_> {
    // Next range [a, b] to query
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((mut u, mut v)) = self.endpoints {
            if self.hld.heads[u] == self.hld.heads[v] {
                if self.hld.depths[u] > self.hld.depths[v] {
                    std::mem::swap(&mut u, &mut v);
                }
                self.endpoints = None;
                Some((self.hld.pos[u], self.hld.pos[v]))
            } else {
                if self.hld.depths[self.hld.heads[u]] > self.hld.depths[self.hld.heads[v]] {
                    std::mem::swap(&mut u, &mut v);
                }
                self.endpoints = Some((u, self.hld.parents[self.hld.heads[v]]));
                Some((self.hld.pos[self.hld.heads[v]], self.hld.pos[v]))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_tree1() -> Graph<()> {
        //         1
        //      /     \
        //      0     2
        //   /     \   \
        //   5     4   3
        // /   \   |
        // 6   7   8
        let mut tree = Graph::new(9, 8);
        tree.connect(1, 0, ());
        tree.connect(1, 2, ());
        tree.connect(0, 5, ());
        tree.connect(0, 4, ());
        tree.connect(2, 3, ());
        tree.connect(5, 6, ());
        tree.connect(5, 7, ());
        tree.connect(4, 8, ());
        tree
    }

    // Tree 1 in bidirectional manner.
    fn test_tree2() -> Graph<()> {
        let mut tree = Graph::new(9, 16);
        tree.connect(1, 0, ());
        tree.connect(1, 2, ());
        tree.connect(0, 5, ());
        tree.connect(0, 4, ());
        tree.connect(2, 3, ());
        tree.connect(5, 6, ());
        tree.connect(5, 7, ());
        tree.connect(4, 8, ());
        tree.connect(0, 1, ());
        tree.connect(2, 1, ());
        tree.connect(5, 0, ());
        tree.connect(4, 0, ());
        tree.connect(3, 2, ());
        tree.connect(6, 5, ());
        tree.connect(7, 5, ());
        tree.connect(8, 4, ());
        tree
    }

    #[test]
    fn fill_chains() {
        let tree = test_tree1();
        let hld = HeavyLight::new(1, &tree);
        assert_eq!(vec![1, 1, 1, 2, 0, 0, 5, 5, 4], hld.parents);
        assert_eq!(vec![2, 1, 2, 3, 3, 3, 4, 4, 4], hld.depths);
        assert_eq!(vec![5, 0, 3, 3, 8, 6, 6, 7, 8], hld.chains);
    }

    #[test]
    fn fill_chains_bidirectional() {
        let tree = test_tree2();
        let hld = HeavyLight::new(1, &tree);
        assert_eq!(vec![1, 1, 1, 2, 0, 0, 5, 5, 4], hld.parents);
        assert_eq!(vec![2, 1, 2, 3, 3, 3, 4, 4, 4], hld.depths);
        assert_eq!(vec![5, 0, 3, 3, 8, 6, 6, 7, 8], hld.chains);
    }

    #[test]
    fn fill_heads() {
        let tree = test_tree1();
        let hld = HeavyLight::new(1, &tree);
        assert_eq!(vec![1, 1, 2, 2, 4, 1, 1, 7, 4], hld.heads);
        assert_eq!(vec![1, 0, 7, 8, 5, 2, 3, 4, 6], hld.pos);
    }

    #[test]
    fn fill_heads_bidirectional() {
        let tree = test_tree2();
        let hld = HeavyLight::new(1, &tree);
        assert_eq!(vec![1, 1, 2, 2, 4, 1, 1, 7, 4], hld.heads);
        assert_eq!(vec![1, 0, 7, 8, 5, 2, 3, 4, 6], hld.pos);
    }

    #[test]
    fn path() {
        let tree = test_tree1();
        let hld = HeavyLight::new(1, &tree);
        assert_eq!(
            vec![(7, 8), (0, 2)],
            hld.path(5, 3).collect::<Vec<(usize, usize)>>()
        );
        assert_eq!(
            vec![(7, 8), (0, 2)],
            hld.path(3, 5).collect::<Vec<(usize, usize)>>()
        );
        assert_eq!(
            vec![(4, 4), (5, 6), (1, 2)],
            hld.path(8, 7).collect::<Vec<(usize, usize)>>()
        );
        assert_eq!(
            vec![(4, 4), (5, 6), (1, 2)],
            hld.path(7, 8).collect::<Vec<(usize, usize)>>()
        );
    }
}
