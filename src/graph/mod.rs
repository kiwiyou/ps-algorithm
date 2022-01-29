pub mod disjoint_set;
pub mod hld;

pub struct Graph<T> {
    nodes: Vec<Option<usize>>,
    edges: Vec<(Option<usize>, usize, T)>,
}

impl<T> Graph<T> {
    pub fn new(n: usize, e: usize) -> Self {
        Self {
            nodes: vec![None; n],
            edges: Vec::with_capacity(e),
        }
    }

    pub fn connect(&mut self, from: usize, to: usize, data: T) {
        let new_edge = self.edges.len();
        let prev = std::mem::replace(&mut self.nodes[from], Some(new_edge));
        self.edges.push((prev, to, data));
    }

    pub fn neighbors(&self, node: usize) -> Neighbors<T> {
        Neighbors {
            graph: self,
            next: self.nodes[node],
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

pub struct Neighbors<'a, T> {
    graph: &'a Graph<T>,
    next: Option<usize>,
}

impl<'a, T> Iterator for Neighbors<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            let (next, ep, data) = &self.graph.edges[next];
            self.next = *next;
            Some((*ep, data))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn connect() {
        let mut graph = Graph::new(2, 2);
        graph.connect(0, 1, ());
        assert_eq!(vec![Some(0), None], graph.nodes);
        assert_eq!(vec![(None, 1, ())], graph.edges);
        graph.connect(0, 0, ());
        assert_eq!(vec![Some(1), None], graph.nodes);
        assert_eq!(vec![(None, 1, ()), (Some(0), 0, ())], graph.edges);
    }

    #[test]
    fn neighbors() {
        let mut graph = Graph::new(3, 2);
        graph.connect(0, 1, ());
        graph.connect(0, 2, ());
        assert_eq!(
            vec![(2, &()), (1, &())],
            graph.neighbors(0).collect::<Vec<(usize, &())>>()
        );
        assert_eq!(
            Vec::<(usize, &())>::new(),
            graph.neighbors(1).collect::<Vec<(usize, &())>>()
        );
    }
}
