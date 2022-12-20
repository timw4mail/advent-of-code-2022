use aoc_shared::enums::Direction;
use aoc_shared::grid::Grid as BaseGrid;
use aoc_shared::impl_grid_newtype;

#[derive(Debug)]
struct Grid<T>(BaseGrid<T>);
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

    fn find_start_and_end(&self) -> (usize, usize) {
        let start = self.vec.iter().position(|item| *item == 'S').unwrap();
        let end = self.vec.iter().position(|item| *item == 'E').unwrap();

        (start, end)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Pathfinder {
    current_elevation: char,
    current_idx: usize,
    start_idx: usize,
    end_idx: usize,
    grid: Grid<char>,
    steps: Vec<Direction>,
}

impl Pathfinder {
    pub fn from_file_str(file_str: &str) -> Self {
        let mut pf = Pathfinder {
            current_elevation: 'a',
            current_idx: 0,
            start_idx: 0,
            end_idx: 0,
            grid: Grid::from_file_str(file_str),
            steps: Vec::new(),
        };
        pf.go_to_start();

        pf
    }

    fn find_start_and_end(&self) -> (usize, usize) {
        (self.start_idx, self.end_idx)
    }

    fn go_to_start(&mut self) {
        let start = self.grid.find_pos('S').unwrap();
        let end = self.grid.find_pos('E').unwrap();

        self.current_idx = start;
        self.start_idx = start;
        self.end_idx = end;
        self.current_elevation = 'a';
        self.grid.vec[start] = 'a';
        self.grid.vec[end] = 'z';
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

    fn is_valid_move(&self, start: usize, dir: Direction) -> bool {
        if let Some(end) = self.get_index_for_move(start, dir) {
            // Is the item within the grid?
            let start_char = self.grid.get(start);
            let end_char = self.grid.get(end);
            if start_char.is_none() || end_char.is_none() {
                return false;
            }

            // Is the elevation change 0 or 1?
            let start_char = *start_char.unwrap();
            let end_char = *end_char.unwrap();
            println!("start, end: {}, {}", start_char, end_char);
            let diff = end_char.to_digit(26).unwrap() - start_char.to_digit(26).unwrap();
            if diff > 1 || end_char < start_char {
                return false;
            }

            let (start_x, start_y) = self.grid.idx_xy(start);
            let (end_x, end_y) = self.grid.idx_xy(end);
            let x_diff = usize::abs_diff(end_x,  start_x);
            let y_diff = usize::abs_diff(end_y, start_y);

            // Have we moved 0 or 1 in a cardinal direction?
            return match (x_diff, y_diff) {
                (0, 0) | (0, 1) | (1,0) => true,
                _ => false,
            };
        };

        false
    }

    fn find_valid_moves(&self, start: usize) -> Vec<usize> {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .into_iter()
            .filter(|d| self.is_valid_move(start, *d))
            .map(|d| self.get_index_for_move(start, d))
            .filter(|m| m.is_some())
            .map(|m| m.unwrap())
            .collect()
    }
}

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let mut finder = Pathfinder::from_file_str(file_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        include_str!("test-input.txt")
    }

    #[test]
    fn get_start_end() {
        let finder = Pathfinder::from_file_str(get_test_data());
        let (start, end) = finder.find_start_and_end();

        assert_eq!((start, end), (0, 21));
    }

    #[test]
    fn find_valid_moves() {
        let finder = Pathfinder::from_file_str(get_test_data());
        let (start, end) = finder.find_start_and_end();

        assert_eq!(finder.find_valid_moves(start), vec![8, 1]);
        assert_eq!(finder.find_valid_moves(8), vec![16, 9]);
    }
}
