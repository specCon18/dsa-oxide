use std::collections::HashMap;

pub struct WeightedAdjacencyList<T, W> {
    vertices: HashMap<T, HashMap<T, W>>,
}

impl<T, W> Clone for WeightedAdjacencyList<T, W>
where
    T: Clone,
    W: Clone,
{
    fn clone(&self) -> Self {
        WeightedAdjacencyList {
            vertices: self.vertices.clone(),
        }
    }
}

impl<T, W> WeightedAdjacencyList<T, W>
where
    T: std::hash::Hash + Eq + Clone,
{
    // Constructor to create a new weighted adjacency list
    pub fn new() -> Self {
        WeightedAdjacencyList {
            vertices: HashMap::new(),
        }
    }

    // Method to add a vertex to the adjacency list
    pub fn add_vertex(&mut self, vertex: T) {
        self.vertices.entry(vertex).or_insert(HashMap::new());
    }

    // Method to add an edge with weight between two vertices
    pub fn add_edge(&mut self, from: T, to: T, weight: W) {
        // Ensure both vertices exist in the adjacency list
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        // Add the edge from 'from' to 'to' with weight
        self.vertices.get_mut(&from).unwrap().insert(to, weight);
    }

    // Method to get the weight of an edge between two vertices
    pub fn get_weight(&self, from: &T, to: &T) -> Option<&W> {
        self.vertices.get(from)?.get(to)
    }

    // Method to get the neighbors of a vertex
    pub fn get_neighbors(&self, vertex: &T) -> Option<&HashMap<T, W>> {
        self.vertices.get(vertex)
    }
    // Method to get the number of vertices in the adjacency list
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}
