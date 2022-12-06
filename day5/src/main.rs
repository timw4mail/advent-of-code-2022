struct Move {
    items: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let items: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        Move { items, from, to }
    }
}

impl Move {
    fn apply(&self, layout: &mut Vec<Vec<char>>) {
        for _ in 0..self.items {
            if let Some(item) = layout[self.from].pop() {
                layout[self.to].push(item);
            }
        }
    }
}

// ----------------------------------------------------------------------------

fn get_initial_layout() -> Vec<Vec<char>> {
    vec![
        Vec::new(),
        vec!['B', 'Q', 'C'],
        vec!['R', 'Q', 'W', 'Z'],
        vec!['B', 'M', 'R', 'L', 'V'],
        vec!['C', 'Z', 'H', 'V', 'T', 'W'],
        vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G'],
        vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q'],
        vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S'],
        vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P'],
        vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P'],
    ]
}

fn get_position_string(layout: &Vec<Vec<char>>) -> String {
    let mut s = String::new();

    for i in 1..=9usize {
        if let Some(ch) = layout[i].last() {
            s.push(*ch);
        }
    }

    s
}

fn main() {
    let file_str = include_str!("input.txt");
    let parts: Vec<&str> = file_str.split("\n\n").collect();
    let moves = parts[1];

    let mut layout = get_initial_layout();

    moves.lines().for_each(|line| {
        Move::from(line).apply(&mut layout);
    });

    let top_crates = get_position_string(&layout);

    println!("Part 1: Top crates after moves: {}", top_crates);
}
