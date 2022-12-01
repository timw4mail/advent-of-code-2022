use std::error::Error;
use std::fs;

fn get_elves(raw: &str) -> Vec<Vec<u32>> {
    raw.split("\n\n")
        .map(|raw_elf| {
            raw_elf
                .split('\n')
                .filter(|value| value.len() > 0)
                .map(move |value| value.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn get_elf_totals(elves: &Vec<Vec<u32>>) -> Vec<u32> {
    elves
        .clone()
        .into_iter()
        .map(|elf| {
            elf.into_iter()
                .reduce(|accum, item| accum + item)
                .unwrap()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_str = fs::read_to_string("input.txt")?;
    let elves = get_elves(&file_str);

    let mut totals: Vec<u32> = get_elf_totals(&elves);
    totals.sort();
    totals.reverse();

    let most = totals[0];
    let top3 = totals[0] + totals[1] + totals[2];

    println!("Part 1: Most calories for one elf: {}", most);
    println!("Part 2: Calories for top three elves: {}", top3);

    Ok(())
}
