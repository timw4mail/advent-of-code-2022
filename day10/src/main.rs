use std::cmp;

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

    fn noop(&self) -> usize {
        1
    }

    fn add_x(&mut self, i: isize) -> usize {
        self.x += i;

        2
    }

    pub fn get_x(&self) -> isize {
        self.x
    }

    pub fn run(&mut self, command: Instruction) -> usize {
        match command {
            Addx(i) => self.add_x(i),
            Noop => self.noop(),
        }
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
enum Pixel {
    Lit,
    Dark,
}
use Pixel::*;

impl Default for Pixel {
    fn default() -> Self {
        Dark
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct CRT {
    pixels: [Pixel; 240],
}

impl CRT {
    fn new() -> Self {
        Self {
            pixels: [Dark; 240],
        }
    }

    fn get_lines(&self) -> Vec<String> {
        self.pixels
            .map(|p| match p {
                Lit => '#',
                Dark => '.',
            })
            .chunks(40)
            .map(|c| String::from_iter(c))
            .collect()
    }

    pub fn draw_pixel(&mut self, cycle: usize, x: isize) {
        let line_x = x % 40;
        let line_cycle = (cycle - 1) % 40;

        let sprite_s = cmp::max(line_x - 1, 0);
        let sprite_e = cmp::min(line_x + 1, 39);
        let sprite: Vec<usize> = ((sprite_s as usize)..=(sprite_e as usize)).collect();

        if cycle == 2 {
            println!(
                "Pixel {}, Sprite {:#?}, Line cycle: {}",
                cycle - 1,
                sprite,
                line_cycle
            );
        }

        if sprite.contains(&(line_cycle)) {
            self.pixels[cycle - 1] = Lit;
        }
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct CycleCounter {
    cpu: CPU,
    crt: CRT,
    log: Vec<isize>,
    cycle: usize,
}

impl CycleCounter {
    pub fn new() -> Self {
        let mut cc = Self {
            cpu: CPU::new(),
            crt: CRT::new(),
            log: vec![1, 1],
            cycle: 1,
        };

        // Do first cycle
        cc.crt.draw_pixel(cc.cycle, cc.cpu.get_x());

        cc
    }

    fn run_line(&mut self, line: &str) {
        let x = self.cpu.get_x();
        let cycles = self.cpu.run(Instruction::from_line(line));

        for _ in 0..(cycles - 1) {
            self.add_cycle(x);
        }

        self.add_cycle(self.cpu.get_x());
    }

    fn add_cycle(&mut self, x: isize) {
        self.cycle += 1;
        self.crt.draw_pixel(self.cycle, x);
        self.log.push(x);
    }

    pub fn display(&self) {
        for line in self.crt.get_lines() {
            println!("{}", line);
        }
    }

    pub fn get_signal_strength(&self, cycle: usize) -> usize {
        let x = self.log.get(cycle).unwrap();

        (*x as usize) * cycle
    }
}

// -----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let mut cc = CycleCounter::new();

    file_str.lines().for_each(|line| cc.run_line(line));

    let sum: usize = [20usize, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|n| cc.get_signal_strength(n))
        .sum();

    println!("Part 1: sum of signal strength: {}", sum);

    println!("Part 2: display output");
    cc.display();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        include_str!("test-input.txt")
    }

    #[test]
    fn test_get_signal_strength() {
        let mut cc = CycleCounter::new();

        get_test_data().lines().for_each(|line| cc.run_line(line));

        assert_eq!(cc.get_signal_strength(20), 420);
        assert_eq!(cc.get_signal_strength(60), 1140);
        assert_eq!(cc.get_signal_strength(100), 1800);
        assert_eq!(cc.get_signal_strength(140), 2940);
        assert_eq!(cc.get_signal_strength(180), 2880);
        assert_eq!(cc.get_signal_strength(220), 3960);
    }

    #[test]
    fn test_crt_get_lines() {
        let file_str = include_str!("test-input.txt");
        let mut cc = CycleCounter::new();

        file_str.lines().for_each(|line| cc.run_line(line));

        let actual = cc.crt.get_lines();

        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..".to_string(),
            "###...###...###...###...###...###...###.".to_string(),
            "####....####....####....####....####....".to_string(),
            "#####.....#####.....#####.....#####.....".to_string(),
            "######......######......######......####".to_string(),
            "#######.......#######.......#######.....".to_string(),
        ];

        assert_eq!(actual, expected);
    }
}
