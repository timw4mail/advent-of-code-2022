mod node;
use core::fmt;
use node::Node;

use aoc_shared::enums::Direction;
use aoc_shared::grid::Grid as BaseGrid;
use aoc_shared::impl_grid_newtype;

#[derive(Debug, Copy, Clone, PartialEq)]
enum CellType {
    Start,
    End,
    Waypoint(u8),
}

#[derive(Debug, Clone)]
struct Cell {
    kind: CellType,
    idx: usize,
    coord: (usize, usize),
}

impl Cell {
    pub fn new(kind: CellType, idx: usize, coord: (usize, usize)) -> Self {
        Cell { kind, idx, coord }
    }

    pub fn get_height(&self) -> u8 {
        match self.kind {
            CellType::Start => 0,
            CellType::End => 25,
            CellType::Waypoint(c) => c,
        }
    }
}

// ----------------------------------------------------------------------------

pub struct Grid<T>(BaseGrid<T>);
impl_grid_newtype!(Grid, BaseGrid<Cell>, Cell);

impl Grid<Cell> {
    pub fn from_file_str(file_str: &str) -> Self {
        let first_line = file_str.lines().next().unwrap();
        let width = first_line.len();

        let mut grid = Grid::new(width);
        let mut idx = 0usize;

        for c in file_str.chars() {
            let kind = match c {
                'S' => CellType::Start,
                'E' => CellType::End,
                'a'..='z' => CellType::Waypoint(c as u8 - b'a'),
                '\r' | '\n' => continue,
                _ => panic!("Invalid character: {c}"),
            };

            let (x, y) = (idx % width, idx / width);
            let cell = Cell::new(kind, idx, (x, y));

            grid.vec.push(cell);

            idx += 1;
        }

        grid
    }

    fn find_pos(&self, value: CellType) -> Option<usize> {
        self.vec.iter().position(|item| item.kind == value)
    }

    pub fn print(&self) {
        for r in 0usize..self.num_rows() {
            let range = self.row_first_idx(r)..=self.row_last_idx(r);
            let line: String = self.vec[range]
                .iter()
                .map(|n| (n.get_height() + b'a') as char)
                .collect();

            println!("{}", line);
        }
    }

    fn get_index_for_move(&self, from: usize, dir: Direction) -> Option<usize> {
        let (mut x, mut y) = self.idx_xy(from);

        match dir {
            Direction::Up => {
                if y >= 1 {
                    y -= 1;
                } else {
                    return None;
                }
            }
            Direction::Down => {
                if y < self.num_rows() - 1 {
                    y += 1;
                } else {
                    return None;
                }
            }
            Direction::Left => {
                if x >= 1 {
                    x -= 1;
                } else {
                    return None;
                }
            }
            Direction::Right => {
                if x < self.num_cols() - 1 {
                    x += 1;
                } else {
                    return None;
                }
            }
        };

        Some(self.xy_idx(x, y))
    }

    fn is_valid_move(&self, start: usize, end: usize) -> bool {
        // Is the item within the grid?
        let start_char = self.get(start);
        let end_char = self.get(end);
        if start_char.is_none() || end_char.is_none() {
            return false;
        }

        let start_char = start_char.unwrap();
        let end_char = end_char.unwrap();
        let start_elevation = start_char.get_height();
        let end_elevation = end_char.get_height();
        if (end_elevation < start_elevation) || end_elevation.abs_diff(start_elevation) > 1 {
            return false;
        }

        let ((start_x, start_y), (end_x, end_y)) = (start_char.coord, end_char.coord);
        let x_diff = end_x.abs_diff(start_x);
        let y_diff = end_y.abs_diff(start_y);

        // Have we moved 0 or 1 in a cardinal direction?
        matches!((x_diff, y_diff), (0, 0) | (0, 1) | (1, 0))
    }

    fn find_moves(&self, start: usize) -> Vec<usize> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|d| self.get_index_for_move(start, d))
        .collect()
    }

    fn find_valid_moves(&self, start: usize) -> Vec<usize> {
        self.find_moves(start)
            .into_iter()
            .filter(|m| self.is_valid_move(start, *m))
            .collect()
    }

    fn has_valid_neighbor(&self, idx: usize) -> bool {
        !self.find_valid_moves(idx).is_empty()
    }
}

impl fmt::Debug for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0usize..self.num_rows() {
            let range = self.row_first_idx(r)..=self.row_last_idx(r);
            let line: String = self.vec[range]
                .iter()
                .map(|n| (n.get_height() + b'a') as char)
                .collect();

            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Pathfinder {
    start_idx: usize,
    end_idx: usize,
    grid: Grid<Cell>,
    tree: Node,
}

impl Pathfinder {
    pub fn from_file_str(file_str: &str) -> Self {
        let mut pf = Pathfinder {
            start_idx: 0,
            end_idx: 0,
            grid: Grid::from_file_str(file_str),
            tree: Node::default(),
        };

        let start = pf.grid.find_pos(CellType::Start).unwrap();
        let end = pf.grid.find_pos(CellType::End).unwrap();

        pf.start_idx = start;
        pf.end_idx = end;
        pf.tree.idx = start;

        pf
    }

    fn add_children(&mut self, node: &mut Node, idx: usize) {
        let possible_moves = self.grid.find_valid_moves(idx);

        if possible_moves.is_empty() {
            return;
        }

        for m in possible_moves {
            if node.contains(m) {
                continue;
            }

            let n = node.add_child(m);
            if m != self.end_idx {
                self.add_children(n, m);
            } else {
                break;
            }
        }
    }

    fn build_tree(&mut self) {
        let mut tree = { self.tree.clone() };
        let idx = { self.start_idx };

        self.add_children(&mut tree, idx);
        self.tree = tree;
    }

    fn get_paths(&self) -> impl Iterator<Item = &Node> {
        self.tree
            .get_leaves()
            .into_iter()
            .filter(|n| n.idx == self.end_idx)
    }

    pub fn find_shortest_path(&mut self) -> &Node {
        self.build_tree();

        self.get_paths()
            .min_by(|a, b| a.get_len().cmp(&b.get_len()))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        include_str!("test-input.txt")
    }

    fn get_finder() -> Pathfinder {
        Pathfinder::from_file_str(get_test_data())
    }

    #[test]
    fn find_valid_moves() {
        let finder = get_finder();

        assert_eq!(finder.grid.find_valid_moves(finder.start_idx), vec![8, 1]);
        assert_eq!(finder.grid.find_valid_moves(8), vec![0, 16, 9]);
    }

    #[test]
    fn find_shortest_path() {
        let mut finder = get_finder();
        let shortest = finder.find_shortest_path();
        assert_eq!(shortest.get_len(), 31);
    }
}
