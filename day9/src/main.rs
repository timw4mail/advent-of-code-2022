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
    head: Location,
    tail: Location,
    head_visited: HashSet<Location>,
    tail_visited: HashSet<Location>,
}

impl Rope {
    pub fn new() -> Self {
        let mut rope = Self::default();
        rope.head_visited.insert(Location::default());
        rope.tail_visited.insert(Location::default());

        rope
    }

    pub fn move_head(&mut self, moves: Move) {
        for _ in 0..moves.amount {
            let mut x = self.head.x;
            let mut y = self.head.y;

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

            self.move_tail(to);
            self.head = to;
            self.head_visited.insert(to);
        }
    }

    fn must_tail_move(&mut self, head: Location) -> bool {
        let distance = self.tail.get_distance(head);

        distance >= 2.0
    }

    fn move_tail(&mut self, head: Location) {
        if !self.must_tail_move(head) {
            return;
        }

        let mut tail = self.tail.clone();

        if tail.y != head.y {
            if head.y - tail.y < 0 {
                tail.y -= 1;
            } else {
                tail.y += 1;
            }
        }

        if tail.x != head.x {
            if head.x - tail.x < 0 {
                tail.x -= 1;
            } else {
                tail.x += 1;
            }
        }

        self.tail = tail;
        self.tail_visited.insert(tail);
    }

    fn get_tail_pos_count(&self) -> usize {
        self.tail_visited.len()
    }
}

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let mut rope = Rope::new();

    file_str
        .lines()
        .map(Move::from_line)
        .for_each(|m| rope.move_head(m));

    let tail_positions = rope.get_tail_pos_count();

    println!("Part 1: Number of tail movements: {}", tail_positions);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> &'static str {
        include_str!("test-input.txt")
    }

    #[test]
    fn test_location_get_distance() {
        let a = Location::new(0, 0);

        assert_eq!(a.get_distance(Location::new(0, 0)), 0.0);
        assert_eq!(a.get_distance(Location::new(1, 0)), 1.0);
        assert_eq!(a.get_distance(Location::new(1, 1)), 2.0f64.sqrt());
    }

    #[test]
    fn test_get_tail_position_count() {
        let mut rope = Rope::new();

        assert_eq!(rope.get_tail_pos_count(), 1);

        get_data()
            .lines()
            .map(Move::from_line)
            .for_each(|m| rope.move_head(m));

        assert_eq!(rope.get_tail_pos_count(), 13);
    }
}
