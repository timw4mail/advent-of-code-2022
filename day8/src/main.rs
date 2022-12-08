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
    fn new(width: usize) -> Self {
        Grid {
            width,
            vec: Vec::new(),
        }
    }

    // Convert x,y coordinate into linear array index
    fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    /// Convert linear array index to x,y coordinate
    fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn row_first_idx(&self, row: usize) -> usize {
        let idx = row * self.width;

        if idx < self.vec.len() {
            idx
        } else {
            self.vec.len()
        }
    }

    fn row_last_idx(&self, row: usize) -> usize {
        if (row + 1) > self.num_rows() {
            return self.vec.len();
        }

        self.row_first_idx(row + 1) - 1
    }

    fn num_rows(&self) -> usize {
        let even_rows = self.vec.len() / self.width;

        if self.vec.len() % self.width > 0 {
            even_rows + 1
        } else {
            even_rows
        }
    }

    fn num_cols(&self) -> usize {
        self.width
    }

    fn get_row(&mut self, row_num: usize) -> &mut [T] {
        let start = self.row_first_idx(row_num);
        let end = self.row_last_idx(row_num);

        &mut self.vec[start..=end]
    }

    fn get_column_indexes(&self, col_num: usize) -> Vec<usize> {
        let mut indexes = Vec::new();

        if col_num > self.num_cols() {
            return indexes;
        }

        for r in 0..self.num_rows() {
            let idx = self.width * r + col_num;
            indexes.push(idx);
        }

        indexes
    }
}

impl Grid<Tree> {
    fn mark_outer_trees_visible(&mut self) {
        fn set_row_visible(row: &mut [Tree]) {
            row.iter_mut().for_each(|tree| { tree.set_all_visible(); })
        }

        // Set top/bottom rows as visible
        set_row_visible(self.get_row(0));
        set_row_visible(self.get_row(self.num_rows() - 1));

        // Set left/right cols as visible
        self.get_column_indexes(0).into_iter().for_each(|id| {
            self.vec[id].set_all_visible();
        });
        self.get_column_indexes(self.num_cols() - 1).into_iter().for_each(|id| {
            self.vec[id].set_all_visible();
        });
    }

    fn mark_visible_trees(&mut self) {
        self.mark_outer_trees_visible();
    }

    pub fn get_visible_trees(&self) -> usize {
        self
            .vec
            .iter()
            .filter(|tree| tree.is_visible())
            .collect::<Vec<&Tree>>()
            .len()
    }

    fn from_file_str(file_str: &'static str) -> Grid<Tree> {
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
    fn test_outer_visible_trees() {
        let mut grid = Grid::from_file_str(get_data());
        grid.mark_outer_trees_visible();

        assert_eq!(grid.get_visible_trees(), 16usize);
    }
}
