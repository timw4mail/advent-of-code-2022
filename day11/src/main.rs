
#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
}

#[derive(Debug)]
struct MonkeyGame {
    monkeys: Vec<Monkey>,
}

fn main() {
    let file_str = include_str!("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        include_str!("test-input.txt")
    }
}