use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

struct Move {
    dir: Direction,
    amount: usize,
}

impl Move {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let dir = match parts[0] {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("Invalid direction!"),
        };
        let amount = parts[1].parse::<usize>().unwrap();

        Move { dir, amount }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, Hash, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Location { x, y }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Rope {
    head: Location,
    tail: Location,
    head_visited: HashSet<Location>,
    tail_visited: HashSet<Location>,
}

impl Rope {
    pub fn new() -> Self {
        Rope { ..Rope::default() }
    }

    pub fn move_head(&mut self, moves: Move) {
        let from = self.head;

        for _ in 0..moves.amount {
            let mut x = from.x;
            let mut y = from.y;

            match moves.dir {
                Up => y += 1,
                Down => y -= 1,
                Left => x -= 1,
                Right => x += 1,
            }

            let to = Location::new(x, y);

            self.move_tail(to);
            self.head = to;
            self.head_visited.insert(to);
        }
    }

    fn move_tail(&mut self, head: Location) {}
}

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let mut rope = Rope::new();

    file_str
        .lines()
        .map(Move::from_line)
        .for_each(|m| rope.move_head(m));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> &'static str {
        include_str!("test-input.txt")
    }
}
