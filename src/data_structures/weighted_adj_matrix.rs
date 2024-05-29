pub struct WeightedAdjacencyMatrix {
    matrix: Vec<Vec<u64>>,
}

impl WeightedAdjacencyMatrix {
    pub fn new(matrix: Vec<Vec<u64>>) -> Self {
        WeightedAdjacencyMatrix { matrix }
    }

    pub fn get_value(&self, row: usize, col: usize) -> Option<u64> {
        self.matrix.get(row).and_then(|r| r.get(col).cloned())
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: u64) -> Result<(), &'static str> {
        if let Some(row_vec) = self.matrix.get_mut(row) {
            if let Some(cell) = row_vec.get_mut(col) {
                *cell = value;
                return Ok(());
            }
        }
        Err("Coordinates out of bounds")
    }

    pub fn transpose(&self) -> WeightedAdjacencyMatrix {
        let transposed = (0..self.matrix[0].len())
            .map(|i| (0..self.matrix.len()).map(|j| self.matrix[j][i]).collect())
            .collect();
        WeightedAdjacencyMatrix::new(transposed)
    }

    pub fn get_row(&self, row_index: usize) -> Option<&Vec<u64>> {
        self.matrix.get(row_index)
    }

    pub fn get_column(&self, col_index: usize) -> Option<Vec<u64>> {
        if col_index >= self.matrix[0].len() {
            return None;
        }
        Some(self.matrix.iter().map(|row| row[col_index]).collect())
    }

    pub fn multiply_scalar(&mut self, scalar: u64) {
        for row in self.matrix.iter_mut() {
            for cell in row.iter_mut() {
                *cell *= scalar;
            }
        }
    }

    pub fn add_matrix(&mut self, other: &WeightedAdjacencyMatrix) -> Option<()> {
        if self.matrix.len() != other.matrix.len() || self.matrix[0].len() != other.matrix[0].len() {
            return None; // Matrices must have the same dimensions to add
        }
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                self.matrix[i][j] += other.matrix[i][j];
            }
        }
        Some(())
    }

    pub fn subtract_matrix(&mut self, other: &WeightedAdjacencyMatrix) -> Option<()> {
        if self.matrix.len() != other.matrix.len() || self.matrix[0].len() != other.matrix[0].len() {
            return None; // Matrices must have the same dimensions to subtract
        }
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                self.matrix[i][j] -= other.matrix[i][j];
            }
        }
        Some(())
    }

    pub fn display_matrix(&self) {
        // Printing the array with row and column indices
        for row in self.matrix.iter() {
            for col in row.iter() {
                print!("{} ", col);
            }
            println!();
        }
    }
}

