fn find_num_chars_before_marker(chars: &Vec<char>) -> usize {
    let mut iter = chars.iter().enumerate().peekable();

    for (i, _) in &mut iter {
        let zero = chars.get(i);
        let one = chars.get(i + 1);
        let two = chars.get(i + 2);
        let three = chars.get(i + 3);

        let mut cursor: Vec<char> = vec![*zero.unwrap(), *one.unwrap(), *two.unwrap(), *three.unwrap()];
        cursor.sort_unstable();

        // If there's a duplicate character, go forward
        if cursor[0] == cursor[1] || cursor[1] == cursor[2] || cursor[2] == cursor[3] || i < 4 {
            continue;
        }

        // Since we are looking 3 characters farther than the current iteration,
        // we need to add that to the returned index, plus an extra one, because
        // the array is zero-indexed
        return i + 4;
    }

    panic!("Marker not found");
}

fn main() {
    let file_str = include_str!("input.txt");
    let chars: Vec<char> = file_str.chars().collect();

    let chars_before_marker = find_num_chars_before_marker(&chars);

    println!(
        "Part 1: Number of characters before start-of-packet marker: {}",
        chars_before_marker
    );
}
