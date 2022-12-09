/// A virtual 2d grid in an ordinary vector. Taken from day 8's puzzle solving implementation
#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    pub vec: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize) -> Self {
        Grid {
            width,
            vec: Vec::new(),
        }
    }

    // Convert x,y coordinate into linear array index
    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    /// Convert linear array index to x,y coordinate
    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.vec.get_mut(idx)
    }

    pub fn row_first_idx(&self, row: usize) -> usize {
        let idx = row * self.width;

        if idx < self.vec.len() {
            idx
        } else {
            self.vec.len()
        }
    }

    pub fn row_last_idx(&self, row: usize) -> usize {
        if (row + 1) > self.num_rows() {
            return self.vec.len();
        }

        self.row_first_idx(row + 1) - 1
    }

    pub fn num_rows(&self) -> usize {
        self.vec.len() / self.width
    }

    pub fn num_cols(&self) -> usize {
        self.width
    }

    pub fn get_row(&mut self, row_num: usize) -> &mut [T] {
        let start = self.row_first_idx(row_num);
        let end = self.row_last_idx(row_num);

        &mut self.vec[start..=end]
    }

    pub fn get_row_indexes(&self, row_num: usize) -> Vec<usize> {
        let start = self.row_first_idx(row_num);
        let end = self.row_last_idx(row_num);

        (start..=end).collect()
    }

    pub fn get_column_indexes(&self, col_num: usize) -> Vec<usize> {
        let mut indexes = Vec::new();

        if col_num >= self.num_cols() {
            panic!(
                "Asked for column {}, there are {} columns",
                col_num,
                self.num_cols()
            );
        }

        for r in 0..self.num_rows() {
            let idx = self.width * r + col_num;
            indexes.push(idx);
        }

        indexes
    }
}