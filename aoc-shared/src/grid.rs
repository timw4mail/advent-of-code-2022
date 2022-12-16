/// Reusing part of day 8's solution for a virtual 2d grid

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    pub vec: Vec<T>,
}

pub trait Grid2d<T> {
    fn new(width: usize) -> Self;

    fn len(&self) -> usize;

    fn num_cols(&self) -> usize;

    fn num_rows(&self) -> usize {
        self.len() / self.num_cols()
    }

    // Convert x,y coordinate into linear array index
    fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * self.num_cols()) + x
    }

    /// Convert linear array index to x,y coordinate
    fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.num_cols(), idx / self.num_cols())
    }

    fn get(&self, idx: usize) -> Option<&T>;

    fn get_mut(&mut self, idx: usize) -> Option<&mut T>;

    fn get_row(&mut self, row_num: usize) -> &mut [T];

    fn row_first_idx(&self, row: usize) -> usize {
        let idx = row * self.num_cols();

        if idx < self.len() {
            idx
        } else {
            self.len()
        }
    }

    fn row_last_idx(&self, row: usize) -> usize {
        if (row + 1) > self.num_rows() {
            return self.len();
        }

        self.row_first_idx(row + 1) - 1
    }

    fn get_row_indexes(&self, row_num: usize) -> Vec<usize> {
        (self.row_first_idx(row_num)..=self.row_last_idx(row_num)).collect()
    }

    fn get_column_indexes(&self, col_num: usize) -> Vec<usize> {
        let mut indexes = Vec::new();

        if col_num >= self.num_cols() {
            panic!(
                "Asked for column {}, there are {} columns",
                col_num,
                self.num_cols()
            );
        }

        for r in 0..self.num_rows() {
            let idx = self.num_cols() * r + col_num;
            indexes.push(idx);
        }

        indexes
    }
}

impl<T> Grid2d<T> for Grid<T> {
    fn new(width: usize) -> Self {
        Grid {
            width,
            vec: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn num_cols(&self) -> usize {
        self.width
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx)
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.vec.get_mut(idx)
    }

    fn get_row(&mut self, row_num: usize) -> &mut [T] {
        let start = self.row_first_idx(row_num);
        let end = self.row_last_idx(row_num);

        &mut self.vec[start..=end]
    }
}
