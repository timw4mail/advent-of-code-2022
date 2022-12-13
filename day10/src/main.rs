use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Addx(isize),
    Noop,
}
use Instruction::*;

impl Instruction {
    pub fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        if parts.len() == 2 && parts[0] == "addx" {
            let value = parts[1].parse::<isize>().unwrap();

            return Addx(value);
        }

        Noop
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct CPU {
    x: isize,
}

impl CPU {
    pub fn new() -> Self {
        CPU { x: 1 }
    }

    fn noop(&self) -> (usize, Option<isize>) {
        (1, None)
    }

    fn addx(&mut self, i: isize) -> (usize, Option<isize>) {
        self.x += i;

        (2, Some(self.x))
    }

    pub fn run(&mut self, command: Instruction) -> (usize, Option<isize>) {
        match command {
            Addx(i) => self.addx(i),
            Noop => self.noop(),
        }
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct CycleCounter {
    cpu: CPU,
    log: HashMap<usize, isize>,
    cycle: usize,
}

impl CycleCounter {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            log: HashMap::new(),
            cycle: 1,
        }
    }
}

// -----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let instructions: Vec<Instruction> = file_str.lines().map(Instruction::from_line).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
}
