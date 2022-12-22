use aoc_shared::enums::Direction;
use aoc_shared::grid::Grid as BaseGrid;
use aoc_shared::impl_grid_newtype;

#[derive(Debug)]
pub struct Grid<T>(BaseGrid<T>);
impl_grid_newtype!(Grid, BaseGrid<char>, char);

impl Grid<char> {
    pub fn from_file_str(file_str: &str) -> Self {
        let lines: Vec<&str> = file_str.lines().collect();
        let width = lines[0].len();

        let mut grid = Grid::new(width);
        lines
            .into_iter()
            .map(|line| line.chars())
            .for_each(|line_chars| grid.vec.append(&mut line_chars.collect::<Vec<char>>()));

        grid
    }

    fn find_pos(&self, value: char) -> Option<usize> {
        self.vec.iter().position(|item| *item == value)
    }

    pub fn print(&self) {
        for r in 0usize..self.num_rows() {
            let range = self.row_first_idx(r)..=self.row_last_idx(r);
            let line: String = self.vec[range].iter().collect();

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

        // Is the character a lowercase letter?
        let start_char = *start_char.unwrap();
        let end_char = *end_char.unwrap();
        if !(start_char.is_ascii_lowercase() && end_char.is_ascii_lowercase()) {
            return false;
        }

        // Is the elevation change 0 or 1?
        let start_char = start_char
            .to_digit(36)
            .expect(&format!("Should be a digit: {}", start_char));
        let end_char = end_char
            .to_digit(36)
            .expect(&format!("Should be a digit: {}", end_char));
        let diff = u32::abs_diff(end_char, start_char);
        if diff > 1  {
            return false;
        }

        let (start_x, start_y) = self.idx_xy(start);
        let (end_x, end_y) = self.idx_xy(end);
        let x_diff = usize::abs_diff(end_x, start_x);
        let y_diff = usize::abs_diff(end_y, start_y);

        // Have we moved 0 or 1 in a cardinal direction?
        match (x_diff, y_diff) {
            (0, 0) | (0, 1) | (1, 0) => true,
            _ => false,
        }
    }

    fn find_moves(&self, start: usize) -> Vec<usize> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
            .into_iter()
            .map(|d| self.get_index_for_move(start, d))
            .filter(|m| m.is_some())
            .map(|m| m.unwrap())
            .collect()
    }

    fn find_valid_moves(&self, start: usize) -> Vec<usize> {
        self.find_moves(start)
            .into_iter()
            .filter(|m| self.is_valid_move(start, *m))
            .collect()
    }

    fn has_valid_neighbor(&self, idx: usize) -> bool {
        self.find_valid_moves(idx).len() > 0
    }

    fn filter_invalid(&mut self, from: usize) {
        let (ch, col_indexes, row_indexes) = {
            let ch = self.get(from).unwrap();
            let (col, row) = self.idx_xy(from);
            let col_indexes = self.get_column_indexes(col);
            let row_indexes = self.get_row_indexes(row);

            (ch, col_indexes, row_indexes)
        };

        self.vec = self.vec
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, c)| {
                if c != *ch {
                    return c;
                }

                return if self.has_valid_neighbor(idx) && (col_indexes.contains(&idx) || row_indexes.contains(&idx)) {
                    c
                } else {
                    '0'
                }
            })
            .collect();

    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct Node {
    idx: usize,
    parents: Vec<usize>,
    children: Option<Vec<Box<Node>>>,
}

impl Node {
    pub fn new(idx: usize) -> Self {
        Node {
            idx,
            ..Node::default()
        }
    }

    pub fn add_child(&mut self, value: usize) -> &mut Self {
        let mut child = Node::new(value);
        child.parents.append(&mut self.parents.clone());
        child.parents.push(self.idx);

        self.append(child);

        self.children
            .as_mut()
            .expect("There should be a Vec here!")
            .last_mut()
            .expect("There should be a Box here!")
    }

    fn append(&mut self, node: Node) -> &mut Self {
        match &mut self.children {
            Some(c) => {
                c.push(Box::new(node));
            }
            None => self.children = Some(vec![Box::new(node)]),
        };

        self
    }

    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn contains(&self, value: usize) -> bool {
        if self.idx == value {
            return true;
        }

        return self.parents.contains(&value);
    }

    pub fn get_leaves(&self) -> Vec<&Node> {
        if self.is_leaf() {
            return vec![self];
        }

        let mut leaves = Vec::new();

        let children = self
            .children
            .as_ref()
            .unwrap()
            .iter()
            .map(|boxed| boxed.as_ref());

        for child in children {
            let mut child_leaves = child.get_leaves();
            leaves.append(&mut child_leaves);
        }

        leaves
    }

    pub fn get_len(&self) -> usize {
        self.parents.len()
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Pathfinder {
    start_idx: usize,
    end_idx: usize,
    grid: Grid<char>,
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
        pf.go_to_start();

        pf
    }

    fn go_to_start(&mut self) {
        let start = self.grid.find_pos('S').unwrap();
        let end = self.grid.find_pos('E').unwrap();

        self.start_idx = start;
        self.end_idx = end;
        self.grid.vec[start] = 'a';
        self.grid.vec[end] = 'z';
        // self.grid.filter_invalid(start);

        self.tree.idx = start;
    }

    fn add_children(&mut self, node: &mut Node, idx: usize) {
        let possible_moves = self.grid.find_valid_moves(idx);

        if possible_moves.len() == 0 {
            return;
        }

        for m in possible_moves {
            if node.contains(m) {
                continue;
            }

            let mut n = node.add_child(m);
            if m != self.end_idx {
                self.add_children(&mut n, m);
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

    fn get_paths(&self) -> impl Iterator<Item= &Node> {
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

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let mut finder = Pathfinder::from_file_str(file_str);
    let shortest_path = finder.find_shortest_path();

    println!("Part 1: Fewest steps: {}", shortest_path.get_len());
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
