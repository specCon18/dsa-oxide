use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::dijkstras_shortest_path;
    #[test]
    fn dijkstras_shortest_path_primeagen_class_test(){
        //      (B) --- (E) ---- (F)
        //    /  |       |       /|
        // (A)   | ------|------- |
        //    \  |/      |        |
        //      (C) --- (D) ---- (G)
        let mut graph:WeightedAdjacencyList<&str,usize> = WeightedAdjacencyList::new();
        graph.add_vertex("A");
        graph.add_vertex("B");
        graph.add_vertex("C");
        graph.add_vertex("D");
        graph.add_vertex("E");
        graph.add_vertex("F");
        graph.add_edge("A","B",3);
        graph.add_edge("A","C",1);
        graph.add_edge("B","A",3);
        graph.add_edge("B","C",4);
        graph.add_edge("B","E",1);
        graph.add_edge("C","B",4);
        graph.add_edge("C","D",7);
        graph.add_edge("C","A",1);
        graph.add_edge("D","C",7);
        graph.add_edge("D","E",5);
        graph.add_edge("D","G",1);
        graph.add_edge("E","B",1);
        graph.add_edge("E","D",5);
        graph.add_edge("E","F",2);
        graph.add_edge("F","G",1);
        graph.add_edge("F","E",2);
        graph.add_edge("F","C",18);
        graph.add_edge("G","D",1);
        graph.add_edge("G","F",1);
        dijkstras_shortest_path(source,sink,graph);
    }
}
// Define a struct to represent the adjacency list with weighted edges
pub struct WeightedAdjacencyList<T, W> {
    vertices: HashMap<T, HashMap<T, W>>,
}

impl<T, W> WeightedAdjacencyList<T, W>
where
    T: std::hash::Hash + Eq + Clone,
{
    // Constructor to create a new weighted adjacency list
    fn new() -> Self {
        WeightedAdjacencyList {
            vertices: HashMap::new(),
        }
    }

    // Method to add a vertex to the adjacency list
    fn add_vertex(&mut self, vertex: T) {
        self.vertices.entry(vertex).or_insert(HashMap::new());
    }

    // Method to add an edge with weight between two vertices
    fn add_edge(&mut self, from: T, to: T, weight: W) {
        // Ensure both vertices exist in the adjacency list
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        // Add the edge from 'from' to 'to' with weight
        self.vertices.get_mut(&from).unwrap().insert(to, weight);
    }

    // Method to get the weight of an edge between two vertices
    fn get_weight(&self, from: &T, to: &T) -> Option<&W> {
        self.vertices.get(from)?.get(to)
    }

    // Method to get the neighbors of a vertex
    fn get_neighbors(&self, vertex: &T) -> Option<&HashMap<T, W>> {
        self.vertices.get(vertex)
    }
}
fn has_unvisited(seen: &[bool], dists: &[f64]) -> bool {
    return seen.iter().enumerate().any(|(i, &s)| !s && dists[i] < f64::INFINITY);
}
fn get_lowest_unvisited(seen: &[bool], dists: &[f64]) -> usize {
    let mut idx = usize::MAX;
    let mut lowest_distance = f64::INFINITY;
    for (i, &is_seen) in seen.iter().enumerate() {
        if is_seen {
            continue;
        }
        if lowest_distance > dists[i] {
            lowest_distance = dists[i];
            idx = i;
        }
    }
    idx
}
pub fn dijkstras_shortest_path(source:usize,sink:usize,mut graph:WeightedAdjacencyList<&str,usize>){
    
}
