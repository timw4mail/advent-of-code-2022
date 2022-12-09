use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
enum VisibleDirection {
    Top,
    Bottom,
    Left,
    Right,
}

use VisibleDirection::*;

#[derive(Debug, Default)]
struct Tree {
    height: usize,
    visible: HashSet<VisibleDirection>,
}

impl Tree {
    fn new(height: usize) -> Self {
        Tree {
            height,
            ..Tree::default()
        }
    }

    fn is_visible(&self) -> bool {
        !self.visible.is_empty()
    }

    fn set_visible(&mut self, dir: VisibleDirection) -> &mut Self {
        self.visible.insert(dir);

        self
    }

    fn set_all_visible(&mut self) -> &mut Self {
        self.set_visible(Top)
            .set_visible(Bottom)
            .set_visible(Left)
            .set_visible(Right)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    vec: Vec<T>,
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

    pub fn get_xy(&self, x: usize, y: usize) -> Option<&T> {
        self.get(self.xy_idx(x, y))
    }

    pub fn get_xy_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.get_mut(self.xy_idx(x, y))
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

impl Grid<Tree> {
    pub fn from_file_str(file_str: &'static str) -> Grid<Tree> {
        let lines: Vec<&str> = file_str.lines().collect();
        let width = lines[0].len();
        let mut grid: Grid<Tree> = Grid::new(width);

        for line in lines {
            let mut row: Vec<Tree> = line
                .chars()
                .map(|ch| Tree::new(ch.to_digit(10).unwrap() as usize))
                .collect();

            grid.vec.append(&mut row);
        }

        grid
    }

    fn mark_outer_trees_visible(&mut self) -> &mut Self {
        fn set_row_visible(row: &mut [Tree]) {
            row.iter_mut().for_each(|tree| {
                tree.set_all_visible();
            })
        }

        // Set top/bottom rows as visible
        set_row_visible(self.get_row(0));
        set_row_visible(self.get_row(self.num_rows() - 1));

        // Set left/right cols as visible
        self.get_column_indexes(0).into_iter().for_each(|id| {
            self.vec[id].set_all_visible();
        });
        self.get_column_indexes(self.num_cols() - 1)
            .into_iter()
            .for_each(|id| {
                self.vec[id].set_all_visible();
            });

        self
    }

    fn mark_visible(&mut self, dir: VisibleDirection) -> &mut Self {
        let indexes: Vec<Vec<usize>> = match dir {
            Top | Bottom => {
                // Skip outer columns, as those are already marked visible
                (1..(self.num_cols() - 1))
                    .map(|c| self.get_column_indexes(c))
                    .map(|column| {
                        if dir == Bottom {
                            column.into_iter().rev().collect()
                        } else {
                            column
                        }
                    })
                    .collect()
            }
            Left | Right => {
                // Skip first and last rows, as those are already marked visible
                (1..(self.num_rows() - 1))
                    .map(|r| self.get_row_indexes(r))
                    .map(|row| {
                        if dir == Right {
                            row.into_iter().rev().collect()
                        } else {
                            row
                        }
                    })
                    .collect()
            }
        };

        for row_or_col in indexes {
            let mut tallest = 0usize;

            for idx in row_or_col {
                let tree = self.get_mut(idx).unwrap();

                if tallest < tree.height {
                    tree.set_visible(dir);

                    tallest = tree.height;
                }
            }
        }

        self
    }

    pub fn mark_visible_trees(&mut self) {
        self.mark_outer_trees_visible()
            .mark_visible(Top)
            .mark_visible(Right)
            .mark_visible(Bottom)
            .mark_visible(Left);
    }

    pub fn get_visible_trees(&self) -> usize {
        self.vec
            .iter()
            .filter(|tree| tree.is_visible())
            .collect::<Vec<&Tree>>()
            .len()
    }

    fn get_surrounding_trees(&self, reference: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
        let (x, y) = self.idx_xy(reference);

        let mut top = self.get_column_indexes(x); // col[0..=x]
        let bottom = top.split_off(y + 1); // col[(x+1)..]
        let mut left = self.get_row_indexes(y); // row[0..=y]
        let right = left.split_off(x + 1); // row[(y+1)..]

        // Remove the index for the current tree
        let _ = top.pop();
        let _ = left.pop();

        // Reverse the top and left so the perspective is from the reference
        top.reverse();
        left.reverse();

        (top, right, bottom, left)
    }

    fn get_viewing_distance(&mut self, reference: usize) -> (usize, usize, usize, usize) {
        let (t, r, b, l) = self.get_surrounding_trees(reference);
        todo!();
    }
}

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let mut grid = Grid::from_file_str(file_str);
    grid.mark_visible_trees();
    let visible_num = grid.get_visible_trees();

    println!("Part 1: Number of visible trees: {}", visible_num);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> &'static str {
        include_str!("test-input.txt")
    }

    #[test]
    fn test_row_first_index() {
        let grid = Grid::from_file_str(get_data());

        assert_eq!(grid.row_first_idx(1), 5);
        assert_eq!(grid.row_first_idx(0), 0);
        assert_eq!(grid.row_first_idx(2), 10);
    }

    #[test]
    fn test_row_last_index() {
        let grid = Grid::from_file_str(get_data());

        assert_eq!(grid.row_last_idx(0), 4);
        assert_eq!(grid.row_last_idx(1), 9);
    }

    #[test]
    fn test_get_column_indexes() {
        let grid = Grid::from_file_str(get_data());

        assert_eq!(grid.width, 5);

        assert_eq!(grid.get_column_indexes(0), vec![0, 5, 10, 15, 20]);
        assert_eq!(grid.get_column_indexes(1), vec![1, 6, 11, 16, 21]);
        assert_eq!(grid.get_column_indexes(4), vec![4, 9, 14, 19, 24]);
    }

    #[test]
    fn test_outer_visible_trees() {
        let mut grid = Grid::from_file_str(get_data());
        grid.mark_outer_trees_visible();

        assert_eq!(grid.get_visible_trees(), 16usize);
    }

    #[test]
    fn test_visible_trees() {
        let mut grid = Grid::from_file_str(get_data());
        grid.mark_visible_trees();

        let visible = [(1usize, 1usize), (2, 1), (1, 2), (4, 3), (2, 3)];

        for (x, y) in visible {
            let idx = grid.xy_idx(x, y);

            assert!(
                grid.vec[idx].is_visible(),
                "Tree {}({},{}) should be visible: {:#?}",
                idx,
                x,
                y,
                grid.vec[idx]
            );
        }

        assert_eq!(grid.get_visible_trees(), 21usize);
    }

    #[test]
    fn test_get_surrounding_trees() {
        let grid = Grid::from_file_str(get_data());
        let (t, r, b, l) = grid.get_surrounding_trees(7);

        assert_eq!(t, vec![2]);
        assert_eq!(r, vec![8, 9]);
        assert_eq!(b, vec![12, 17, 22]);
        assert_eq!(l, vec![6, 5]);
    }
}
