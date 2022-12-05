fn get_split_rucksack_items(list: &str) -> (Vec<char>, Vec<char>) {
    let all_chars: Vec<_> = list.chars().collect();

    let half_index = all_chars.len() / 2;
    let halves = all_chars.split_at(half_index);

    assert_eq!(halves.0.len(), halves.1.len());

    (halves.0.to_vec(), halves.1.to_vec())
}

fn get_rucksack_items(list: &str) -> Vec<char> {
    list.chars().collect()
}

fn get_priority(item: char) -> u32 {
    let is_uppercase = item.is_ascii_uppercase();
    let char = if is_uppercase {
        item.to_ascii_lowercase()
    } else {
        item
    };

    let priority = match char {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0,
    };

    if is_uppercase {
        priority + 26
    } else {
        priority
    }
}

fn find_same_item(a: Vec<char>, b: Vec<char>) -> char {
    for ch in a {
        if b.contains(&ch) {
            return ch;
        }
    }

    panic!("We should have already found the item")
}

fn find_same_item_in_three(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>) -> char {
    for ch in a {
        if b.contains(&ch) && c.contains(&ch) {
            return *ch;
        }
    }

    panic!("We should have already found the common item");
}

fn main() {
    let file_str = include_str!("input.txt");
    let sum: u32 = file_str
        .lines()
        .map(|line| get_split_rucksack_items(line))
        .map(|(a, b)| find_same_item(a, b))
        .map(|item| get_priority(item))
        .sum();

    let sacks: Vec<Vec<char>> = file_str
        .lines()
        .map(|line| get_rucksack_items(line))
        .collect();
    let group_sum: u32 = sacks
        .chunks(3)
        .map(|chunk| find_same_item_in_three(&chunk[0], &chunk[1], &chunk[2]))
        .map(|item| get_priority(item))
        .sum();

    println!("Part 1 Priority Sum: {}", sum);
    println!("Part 2 Group Sum: {}", group_sum);
}
