use crate::data_structures::{weighted_adj_list::WeightedAdjacencyList,min_heap::MinHeap};
#[cfg(test)]
mod tests {
    use crate::data_structures::weighted_adj_list::WeightedAdjacencyList;
    use super::dijkstras_shortest_path;
    #[test]
    fn dijkstras_shortest_path_primeagen_class_test(){
        //      (1) --- (4) ---- (5)
        //    /  |       |       /|
        // (0)   | ------|------- |
        //    \  |/      |        |
        //      (2) --- (3) ---- (6)
        let source = 0;
        let sink = 5;

        let mut graph:WeightedAdjacencyList<usize,usize> = WeightedAdjacencyList::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_vertex(4);
        graph.add_vertex(5);
        graph.add_edge(0, 1, 3);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 0, 3);
        graph.add_edge(1, 2, 4);
        graph.add_edge(1, 4, 1);
        graph.add_edge(2, 1, 4);
        graph.add_edge(2, 3, 7);
        graph.add_edge(2, 0, 1);
        graph.add_edge(3, 2, 7);
        graph.add_edge(3, 4, 5);
        graph.add_edge(3, 6, 1);
        graph.add_edge(4, 1, 1);
        graph.add_edge(4, 3, 5);
        graph.add_edge(4, 5, 2);
        graph.add_edge(5, 6, 1);
        graph.add_edge(5, 4, 2);
        graph.add_edge(5, 2, 18);
        graph.add_edge(6, 3, 1);
        graph.add_edge(6, 5, 1);
        let path = dijkstras_shortest_path(source,sink,graph.clone());
        assert_eq!(path.unwrap(), vec![0, 1, 4, 5]);
    }
}

pub fn dijkstras_shortest_path( source: usize, sink: usize, graph: WeightedAdjacencyList<usize, usize>) -> Option<Vec<usize>> {
    let mut visited: MinHeap<(f64, usize)> = MinHeap::new();
    let mut prev: Vec<usize> = vec![usize::MAX; graph.len()];
    let mut dists: Vec<f64> = vec![f64::INFINITY; graph.len()];
    dists[source] = 0.0;
    visited.push((0.0, source));

    while let Some((_, curr)) = visited.pop() {
        if curr == sink {
            break; // found shortest path to sink
        }

        if let Some(adjs) = graph.get_neighbors(&curr) {
            for (&to, &weight) in adjs.iter() {
                let new_dist = dists[curr] + weight as f64;
                if new_dist < dists[to] {
                    dists[to] = new_dist;
                    prev[to] = curr;
                    visited.push((new_dist, to));
                }
            }
        }
    }

    // Check if sink is unreachable
    if prev[sink] == usize::MAX {
        return None;
    }

    // Reconstruct shortest path
    let mut path = Vec::new();
    let mut curr = sink;
    while curr != usize::MAX {
        path.push(curr);
        curr = prev[curr];
    }
    path.reverse();
    Some(path)
}
fn has_unvisited(seen: &[bool], dists: &[f64]) -> bool {
    seen.iter().enumerate().any(|(i, &s)| !s && dists[i] < f64::INFINITY)
}

fn get_lowest_unvisited(seen: &[bool], dists: &[f64]) -> usize {
    let mut idx = usize::MAX;
    let mut lowest_distance = f64::INFINITY;
    for (i, &is_seen) in seen.iter().enumerate() {
        if !is_seen && dists[i] < lowest_distance {
            lowest_distance = dists[i];
            idx = i;
        }
    }
    idx
}
