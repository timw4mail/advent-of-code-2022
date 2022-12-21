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

    pub fn get_leaves(&self) -> Vec<Node> {
        if self.is_leaf() {
            return vec![self.to_owned()];
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
    current_elevation: char,
    start_idx: usize,
    end_idx: usize,
    grid: Grid<char>,
    tree: Node,
}

impl Pathfinder {
    pub fn from_file_str(file_str: &str) -> Self {
        let mut pf = Pathfinder {
            current_elevation: 'a',
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
        self.current_elevation = 'a';
        self.grid.vec[start] = 'a';
        self.grid.vec[end] = 'z';
        self.tree.idx = start;
    }

    fn get_index_for_move(&self, from: usize, dir: Direction) -> Option<usize> {
        let (mut x, mut y) = self.grid.idx_xy(from);

        match dir {
            Direction::Up => {
                if y >= 1 {
                    y -= 1;
                } else {
                    return None;
                }
            }
            Direction::Down => {
                if y < self.grid.num_rows() - 1 {
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
                if x < self.grid.num_cols() - 1 {
                    x += 1;
                } else {
                    return None;
                }
            }
        };

        Some(self.grid.xy_idx(x, y))
    }

    fn is_valid_move(&self, start: usize, end: usize) -> bool {
        // Is the item within the grid?
        let start_char = self.grid.get(start);
        let end_char = self.grid.get(end);
        if start_char.is_none() || end_char.is_none() {
            return false;
        }

        // Is the elevation change 0 or 1?
        let start_char = *start_char.unwrap();
        let end_char = *end_char.unwrap();
        let start_char = start_char
            .to_digit(36)
            .expect(&format!("Should be a digit: {}", start_char)) as i32;
        let end_char = end_char
            .to_digit(36)
            .expect(&format!("Should be a digit: {}", end_char)) as i32;
        let diff = end_char - start_char;
        if diff > 1 || diff < 0 || end_char < start_char {
            return false;
        }

        let (start_x, start_y) = self.grid.idx_xy(start);
        let (end_x, end_y) = self.grid.idx_xy(end);
        let x_diff = usize::abs_diff(end_x, start_x);
        let y_diff = usize::abs_diff(end_y, start_y);

        // Have we moved 0 or 1 in a cardinal direction?
        match (x_diff, y_diff) {
            (0, 0) | (0, 1) | (1, 0) => true,
            _ => false,
        }
    }

    fn find_valid_moves(&self, start: usize) -> Vec<usize> {
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
        .filter(|m| self.is_valid_move(start, *m))
        .collect()
    }

    fn add_children(&mut self, node: &mut Node, idx: usize) {
        let possible_moves = self.find_valid_moves(idx);

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

    fn get_paths(&self) -> Vec<Node> {
        self.tree
            .get_leaves()
            .into_iter()
            .filter(|n| n.idx == self.end_idx)
            .collect()
    }

    pub fn find_shortest_path(&mut self) -> Node {
        self.build_tree();

        self.get_paths()
            .iter()
            .min_by(|a, b| a.get_len().cmp(&b.get_len()))
            .unwrap()
            .to_owned()
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

        assert_eq!(finder.find_valid_moves(finder.start_idx), vec![8, 1]);
        assert_eq!(finder.find_valid_moves(8), vec![0, 16, 9]);
    }

    #[test]
    fn find_shortest_path() {
        let shortest = get_finder().find_shortest_path();

        assert_eq!(shortest.get_len(), 31);
    }
}
