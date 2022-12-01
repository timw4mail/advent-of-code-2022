use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_str = fs::read_to_string("input.txt")?;
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut totals: Vec<u32> = Vec::new();

    for raw_elf in file_str.split("\n\n") {
        let elf: Vec<u32> = raw_elf.split('\n')
            .filter(|value| value.len() > 0)
            .map(move |value| {
                value.parse::<u32>().unwrap()
            })
            .collect();

        let sum = elf
            .clone()
            .into_iter()
            .reduce(|accum, item| accum + item)
            .unwrap();


        elves.push(elf);
        totals.push(sum);
    }

    let most = totals.iter().max().unwrap();

    println!("{:?}{:?}", elves, totals);
    println!("Max calories: {}", most);

    Ok(())
}
