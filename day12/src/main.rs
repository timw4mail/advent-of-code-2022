fn main() {
    let file_str = include_str!("input.txt");
    let mut finder = ::day12::Pathfinder::from_file_str(file_str);
    let shortest_path = finder.find_shortest_path();

    println!("Part 1: Fewest steps: {}", shortest_path.get_len());
}
