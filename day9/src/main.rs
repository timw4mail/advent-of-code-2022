mod grid;

use grid::Grid;

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> &'static str {
        include_str!("test-input.txt")
    }
}