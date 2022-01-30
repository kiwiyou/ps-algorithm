pub struct Graph<T> {
    list: Vec<Vec<(usize, T)>>,
    edges: usize,
}

impl<T> Graph<T> {
    pub fn new(n: usize) -> Self {
        Self {
            list: (0..n).map(|_| vec![]).collect(),
            edges: 0,
        }
    }

    pub fn connect(&mut self, from: usize, to: usize, data: T) {
        self.list[from].push((to, data));
        self.edges += 1;
    }

    pub fn neighbors(&self, node: usize) -> Neighbors<T> {
        Neighbors(self.list[node].iter())
    }

    pub fn node_count(&self) -> usize {
        self.list.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges
    }
}

pub struct Neighbors<'a, T>(std::slice::Iter<'a, (usize, T)>);

impl<'a, T> Iterator for Neighbors<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (node, data) = self.0.next()?;
        Some((*node, data))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn connect() {
        let mut graph = Graph::new(2);
        graph.connect(0, 1, ());
        assert_eq!(vec![vec![(1, ())], vec![]], graph.list);
        graph.connect(0, 0, ());
        assert_eq!(vec![vec![(1, ()), (0, ())], vec![]], graph.list);
    }

    #[test]
    fn neighbors() {
        let mut graph = Graph::new(3);
        graph.connect(0, 1, ());
        graph.connect(0, 2, ());
        assert_eq!(
            vec![(1, &()), (2, &())],
            graph.neighbors(0).collect::<Vec<(usize, &())>>()
        );
        assert_eq!(
            Vec::<(usize, &())>::new(),
            graph.neighbors(1).collect::<Vec<(usize, &())>>()
        );
    }
}
