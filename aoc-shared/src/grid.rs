/// Reusing part of day 8's solution for a virtual 2d grid

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

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn num_cols(&self) -> usize {
        self.width
    }

    pub fn num_rows(&self) -> usize {
        self.len() / self.num_cols()
    }

    // Convert x,y coordinate into linear array index
    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * self.num_cols()) + x
    }

    /// Convert linear array index to x,y coordinate
    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.num_cols(), idx / self.num_cols())
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.vec.get_mut(idx)
    }

    pub fn get_row(&mut self, row_num: usize) -> &mut [T] {
        let start = self.row_first_idx(row_num);
        let end = self.row_last_idx(row_num);

        &mut self.vec[start..=end]
    }

    pub fn row_first_idx(&self, row: usize) -> usize {
        let idx = row * self.num_cols();

        if idx < self.len() {
            idx
        } else {
            self.len()
        }
    }

    pub fn row_last_idx(&self, row: usize) -> usize {
        if (row + 1) > self.num_rows() {
            return self.len();
        }

        self.row_first_idx(row + 1) - 1
    }

    pub fn get_row_indexes(&self, row_num: usize) -> Vec<usize> {
        (self.row_first_idx(row_num)..=self.row_last_idx(row_num)).collect()
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
            let idx = self.num_cols() * r + col_num;
            indexes.push(idx);
        }

        indexes
    }
}

#[macro_export]
/// Simplifies newtype wrapping of the `Grid` struct
macro_rules! impl_grid_newtype {
    ($($struct: tt, $target: path, $type: ty),* ) => {
        $(
            impl ::core::ops::Deref for $struct<$type> {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::core::ops::DerefMut for $struct<$type> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl $struct<$type> {
                pub fn new(width: usize) -> Self {
                    $struct(<$target>::new(width))
                }
            }
        )*
    }
}
