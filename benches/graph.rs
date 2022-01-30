use criterion::{criterion_group, criterion_main, Criterion, black_box, BenchmarkId, BatchSize};
use ps_algorithm::data_structure::Graph;
use rand::{thread_rng, Rng};

fn make_tree(n: usize) -> Graph<()> {
    let mut rng = thread_rng();
    let mut graph = Graph::new(n);
    for i in 1..n {
        let j = rng.gen_range(0..i);
        graph.connect(i, j, ());
        graph.connect(j, i, ());
    }
    graph
}

fn make_complete_graph(n: usize) -> Graph<()> {
    let mut graph = Graph::new(n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                graph.connect(i, j, ());
            }
        }
    }
    graph
}

fn dfs(visited: &mut [bool], root: usize, graph: &Graph<()>) {
    for (next, _) in graph.neighbors(root) {
        if !visited[next] {
            visited[next] = true;
            black_box(next);
            dfs(visited, next, graph);
        }
    }
}

pub fn graph_dfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_dfs");
    for n in [100, 1000, 10000, 100000] {
        let tree = make_tree(n);
        group.bench_with_input(BenchmarkId::from_parameter(format!("{} tree", n)), &tree, |b, tree| {
            b.iter_batched(|| vec![false; tree.node_count()], |mut visited| dfs(&mut visited, 0, tree), BatchSize::LargeInput);
        });
    }
    for n in [10, 100, 1000] {
        let complete = make_complete_graph(n);
        group.bench_with_input(BenchmarkId::from_parameter(format!("{} complete", n)), &complete, |b, complete| {
            b.iter_batched(|| vec![false; complete.node_count()], |mut visited| dfs(&mut visited, 0, complete), BatchSize::LargeInput);
        });
    }
}

criterion_group!(benches, graph_dfs);
criterion_main!(benches);
