use std::collections::{HashMap, HashSet};

use algo_examples::formatting::on_screen_len;
use algo_examples::shorthands::new_h_map;

type NodeGraph<K, V> = HashMap<K, HashMap<K, V>>;

fn main() {
    let (start, a, b, finish) = ("start", "a", "b", "fin");
    let (start_neighbors, b_neighbors, a_neighbors, finish_neighbors) = (
        new_h_map([(a, 6), (b, 2)]),
        new_h_map([(a, 3), (finish, 5)]),
        new_h_map([(finish, 1)]),
        new_h_map([]),
    );
    let graph = NodeGraph::from([
        (start, start_neighbors),
        (b, b_neighbors),
        (a, a_neighbors),
        (finish, finish_neighbors),
    ]);

    let expected = 6;
    let shortest_path = dejkstras_alg(&graph, start, finish).unwrap();

    let print = if shortest_path <= expected {
        format!("✨ It works! Answer is {shortest_path} ✅")
    } else {
        format!("🚧 Oh, shieeet, answer is {shortest_path} instead of {expected} ❌")
    };
    let border_len = on_screen_len(&print) + 4;
    let border = String::from_iter(vec!['-'; border_len]);
    println!("{border}\n| {print} |\n{border}");
}

use std::hash::Hash;

fn find_lowest_cost_node<'a, K: Eq + Hash + ?Sized>(
    costs: &HashMap<&'a K, i32>,
    processed: &HashSet<&K>,
) -> Option<&'a K> {
    let (mut lowest_cost, mut lowest_cost_node) = (i32::MAX, None);

    for (&node, &cost) in costs {
        if !processed.contains(node) && cost < lowest_cost {
            (lowest_cost, lowest_cost_node) = (cost, Some(node))
        }
    }

    lowest_cost_node
}

/// Dejkstra's algorithm implementation used to find the
/// shortest path in a weighted graph.
///
/// [!!] Cannot be used with negative weights. [!!]
pub fn dejkstras_alg<K: Eq + Hash + ?Sized>(
    graph: &NodeGraph<&K, i32>,
    start: &K,
    finish: &K,
) -> Option<i32> {
    let mut costs = graph.get(start)?.clone();
    let (mut parents, mut processed) = (HashMap::new(), HashSet::new());

    let mut opt_node = find_lowest_cost_node(&costs, &processed);
    while let Some(node) = opt_node {
        let cost = costs[node];
        let neighbors = graph.get(node)?;

        for n in neighbors.keys() {
            let new_cost = cost + neighbors[n];
            let old_cost = *costs.entry(n).or_insert(i32::MAX);
            if new_cost < old_cost {
                costs.insert(n, new_cost);
                parents.insert(n, node);
            }
        }

        processed.insert(node);
        opt_node = find_lowest_cost_node(&costs, &processed);
    }

    costs.get(finish).copied()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use algo_examples::benchmarking::{bench_once, bench_times, calc_iterations};
    use algo_examples::formatting::{is_emoji, on_screen_len};

    use super::*;

    #[test]
    fn dejkstras_algorithm_test() {
        let (start, finish, a, b, c, d) = ("start", "finish", "a", "b", "c", "d");

        let mut graph = NodeGraph::new();
        graph.insert(start, new_h_map([(a, 6), (b, 2)]));
        graph.insert(b, new_h_map([(a, 3), (finish, 5)]));
        graph.insert(a, new_h_map([(finish, 1)]));
        graph.insert(finish, HashMap::new());
        let res = dejkstras_alg(&graph, start, finish);
        assert_eq!(res, Some(6));

        let mut graph = NodeGraph::new();
        graph.insert(start, new_h_map([(a, 5), (b, 2)]));
        graph.insert(a, new_h_map([(c, 4), (d, 2)]));
        graph.insert(b, new_h_map([(a, 8), (d, 7)]));
        graph.insert(c, new_h_map([(finish, 3), (d, 6)]));
        graph.insert(d, new_h_map([(finish, 1)]));
        graph.insert(finish, HashMap::new());
        let res = dejkstras_alg(&graph, start, finish);
        assert!(res <= Some(8));

        let mut graph = NodeGraph::new();
        graph.insert(start, new_h_map([(a, 10)]));
        graph.insert(a, new_h_map([(c, 20)]));
        graph.insert(b, new_h_map([(a, 1)]));
        graph.insert(c, new_h_map([(b, 1), (finish, 30)]));
        graph.insert(finish, HashMap::new());
        let res = dejkstras_alg(&graph, start, finish);
        assert!(res <= Some(60));

        // This test contains negative numbers, but the book notes explain that..
        // ..in this example negative number is small and so it gets "outweighted".
        let mut graph = NodeGraph::new();
        graph.insert(start, new_h_map([(a, 2), (b, 2)]));
        graph.insert(a, new_h_map([(b, 2)]));
        graph.insert(b, new_h_map([(c, 2), (finish, 2)]));
        graph.insert(c, new_h_map([(b, -1), (finish, 2)]));
        graph.insert(finish, HashMap::new());
        let res = dejkstras_alg(&graph, start, finish);
        assert!(res <= Some(4));
    }

    #[test]
    fn bench() {
        let (start, finish, a, b) = ("start", "finish", "a", "b");

        let mut graph = NodeGraph::new();
        graph.insert(start, new_h_map([(a, 6), (b, 2)]));
        graph.insert(b, new_h_map([(a, 3), (finish, 5)]));
        graph.insert(a, new_h_map([(finish, 1)]));
        graph.insert(finish, HashMap::new());

        let one_measurment_takes = bench_once(|| dejkstras_alg(&graph, start, finish));
        let times = calc_iterations(one_measurment_takes, Duration::from_secs(7));

        let res = bench_times(times, || dejkstras_alg(&graph, start, finish)).unwrap();
        assert!(res < Duration::from_micros(25));
    }
}
