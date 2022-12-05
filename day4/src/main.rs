#[derive(Debug, PartialEq, PartialOrd)]
struct Range {
    low: u32,
    high: u32,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split("-").collect();

        let low: u32 = parts[0].parse().unwrap();
        let high: u32 = parts[1].parse().unwrap();

        Range { low, high }
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.low <= other.low && self.high >= other.high
    }
}

// ----------------------------------------------------------------------------

fn parse_ranges(line: &str) -> (Range, Range) {
    let ranges: Vec<&str> = line.split(',').collect();
    let range_a = Range::from(ranges[0]);
    let range_b = Range::from(ranges[1]);

    (range_a, range_b)
}

fn main() {
    let file_str = include_str!("input.txt");

    let count = file_str
        .lines()
        .map(|line| parse_ranges(line))
        .map(|(range_a, range_b)| range_a.contains(&range_b) || range_b.contains(&range_a))
        .filter(|contains| *contains == true)
        .collect::<Vec<bool>>()
        .len();

    println!("Part 1: fully contained pairs: {}", count);
}
