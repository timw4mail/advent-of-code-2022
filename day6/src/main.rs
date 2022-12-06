fn find_num_chars_before_marker(chars: &Vec<char>, marker_size: usize) -> usize {
    let mut iter = chars.iter().enumerate();

    'outer: for (i, _) in &mut iter {
        // Look marker_size -1 characters forward so we can check for duplicate characters
        let mut cursor: Vec<char> = Vec::new();
        for n in 0..marker_size {
            cursor.push(*chars.get(i + n).unwrap());
        }
        cursor.sort_unstable();

        // If there's a duplicate character, go forward
        for (n, ch) in cursor.iter().enumerate() {
            if let Some(other) = cursor.get(n + 1) {
                if ch == other {
                    continue 'outer;
                }
            }
        }

        // Since we are looking farther than the current iteration,
        // we need to add the marker size that to the returned index
        return i + marker_size;
    }

    panic!("Marker not found");
}

fn find_packet_marker(chars: &Vec<char>) -> usize {
    find_num_chars_before_marker(chars, 4)
}

fn find_message_marker(chars: &Vec<char>) -> usize {
    find_num_chars_before_marker(chars, 14)
}

fn main() {
    let file_str = include_str!("input.txt");
    let chars: Vec<char> = file_str.chars().collect();

    println!(
        "Part 1: Number of characters before start-of-packet marker: {}",
        find_packet_marker(&chars)
    );

    println!(
        "Part 2: Number of characters before start-of-message marker: {}",
        find_message_marker(&chars)
    );
}
