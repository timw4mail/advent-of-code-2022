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
    grid: Grid<char>,
    steps: Vec<Direction>,
}

impl Pathfinder {
    pub fn from_file_str(file_str: &str) -> Self {
        Pathfinder {
            grid: Grid::from_file_str(file_str),
            steps: Vec::new(),
        }
    }

    fn find_start_and_end(&self) -> (usize, usize) {
        (self.grid.find_pos('S').unwrap(), self.grid.find_pos('E').unwrap())
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
}
