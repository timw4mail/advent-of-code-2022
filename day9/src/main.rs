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
    amount: isize,
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
        let amount = parts[1].parse::<isize>().unwrap();

        Move { dir, amount }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, Hash, PartialEq)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn new(x: isize, y: isize) -> Self {
        Location { x, y }
    }

    fn get_distance(self, other: Self) -> f64 {
        let squares = (other.x - self.x).pow(2) + (other.y - self.y).pow(2);

        (squares as f64).sqrt()
    }
}

// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Rope {
    knots: Vec<Location>,
    knot_count: usize,
    head_visited: HashSet<Location>,
    tail_visited: HashSet<Location>,
}

impl Rope {
    pub fn new(knot_count: usize) -> Self {
        let mut rope = Self::default();
        rope.knot_count = knot_count;
        rope.head_visited.insert(Location::default());
        rope.tail_visited.insert(Location::default());
        rope.knots.resize_with(knot_count, Location::default);

        rope
    }

    pub fn get_knot(&self, idx: usize) -> Location {
        self.knots[idx]
    }

    pub fn is_tail(&self, idx: usize) -> bool {
        idx == (self.knot_count - 1)
    }

    pub fn move_head(&mut self, moves: Move) {
        for _ in 0..moves.amount {
            let mut x = self.knots[0].x;
            let mut y = self.knots[0].y;

            match moves.dir {
                Up => {
                    y += 1;
                }
                Down => {
                    y -= 1;
                }
                Left => {
                    x -= 1;
                }
                Right => {
                    x += 1;
                }
            }

            let to = Location::new(x, y);
            self.knots[0] = to;

            for i in 1..self.knot_count {
                self.move_knot(i, i - 1);
            }
            // self.head = to;
            // self.head_visited.insert(to);
        }
    }

    fn must_move(&mut self, current: usize, prev: usize) -> bool {
        let distance = self.get_knot(current).get_distance(self.get_knot(prev));

        distance >= 2.0
    }

    fn move_knot(&mut self, c: usize, p: usize) {
        if !self.must_move(c, p) {
            return;
        }

        let mut current = self.get_knot(c);
        let prev = self.get_knot(p);

        if current.y != prev.y {
            if prev.y - current.y < 0 {
                current.y -= 1;
            } else {
                current.y += 1;
            }
        }

        if current.x != prev.x {
            if prev.x - current.x < 0 {
                current.x -= 1;
            } else {
                current.x += 1;
            }
        }

        self.knots[c] = current;
        if self.is_tail(c) {
            self.tail_visited.insert(current);
        }
    }

    fn get_tail_pos_count(&self) -> usize {
        self.tail_visited.len()
    }
}

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");

    let mut rope = Rope::new(2);

    file_str
        .lines()
        .map(Move::from_line)
        .for_each(|m| rope.move_head(m));

    let tail_positions = rope.get_tail_pos_count();

    println!(
        "Part 1: Number of tail movements with 2 knots: {}",
        tail_positions
    );

    let mut rope = Rope::new(10);

    file_str
        .lines()
        .map(Move::from_line)
        .for_each(|m| rope.move_head(m));

    let tail_positions = rope.get_tail_pos_count();

    println!(
        "Part 2: Number of tail movements with 10 knots: {}",
        tail_positions
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_get_distance() {
        let a = Location::new(0, 0);

        assert_eq!(a.get_distance(Location::new(0, 0)), 0.0);
        assert_eq!(a.get_distance(Location::new(1, 0)), 1.0);
        assert_eq!(a.get_distance(Location::new(1, 1)), 2.0f64.sqrt());
    }

    #[test]
    fn test_get_tail_position_count() {
        let file_str = include_str!("test-input.txt");
        let mut rope = Rope::new(2);

        file_str
            .lines()
            .map(Move::from_line)
            .for_each(|m| rope.move_head(m));

        assert_eq!(rope.get_tail_pos_count(), 13);
    }

    #[test]
    fn test_get_tail_position_count_10_knots() {
        let file_str = include_str!("test-input2.txt");
        let mut rope = Rope::new(10);

        file_str
            .lines()
            .map(Move::from_line)
            .for_each(|m| rope.move_head(m));

        assert_eq!(rope.get_tail_pos_count(), 36);
    }
}
