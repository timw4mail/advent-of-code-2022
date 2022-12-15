use std::collections::VecDeque;
use std::ops::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum WorryType {
    Normal,
    Extra,
}

#[derive(Debug)]
struct Operation {
    operator: char,
    operand: String,
}

impl Operation {
    fn run(&self, old: u128) -> u128 {
        let other = match self.operand.as_str() {
            "old" => old,
            _ => self.operand.parse::<u128>().unwrap(),
        };

        match self.operator {
            '+' => old.add(other),
            '-' => old.sub(other),
            '*' => old.mul(other),
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: usize,
    pass_monkey: usize,
    fail_monkey: usize,
    inspection_count: u128,
    inspection_worry: WorryType,
}

impl Monkey {
    fn from_behavior(raw: &str, inspection_worry: WorryType) -> Self {
        let lines: Vec<&str> = raw.lines().collect();

        let item_parts: Vec<&str> = lines[1].split(": ").collect();
        let items: VecDeque<u128> = item_parts[1]
            .split(", ")
            .map(|i| i.parse::<u128>().unwrap())
            .collect();

        let op_parts: Vec<&str> = lines[2].split(" = ").collect();
        let [_, operator, operand]: [&str; 3] = op_parts[1]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();

        let [test, pass_monkey, fail_monkey]: [usize; 3] = lines[3..]
            .iter()
            .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
            .map(|parts| parts[parts.len() - 1])
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        Monkey {
            items,
            operation: Operation {
                operator: operator.chars().next().unwrap(),
                operand: operand.to_string(),
            },
            test,
            pass_monkey,
            fail_monkey,
            inspection_count: 0,
            inspection_worry,
        }
    }

    fn run_test(&self, item: u128) -> usize {
        if item % (self.test as u128) == 0 {
            self.pass_monkey
        } else {
            self.fail_monkey
        }
    }

    pub fn inspect(&mut self, item: u128) -> (usize, u128) {
        self.inspection_count += 1;

        let worry = if self.inspection_worry == WorryType::Normal {
            self.operation.run(item) / 3
        } else {
            self.operation.run(item)
        };

        let new_monkey = self.run_test(worry);

        (new_monkey, worry)
    }

    pub fn catch(&mut self, item: u128) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
struct MonkeyGame {
    monkeys: Vec<Monkey>,
}

impl MonkeyGame {
    pub fn from_file_str(file_str: &'static str, inspection_worry: WorryType) -> Self {
        let behaviors = file_str.split("\n\n");

        Self {
            monkeys: behaviors
                .map(|m| Monkey::from_behavior(m, inspection_worry))
                .collect(),
        }
    }

    fn throw(&mut self, item: u128, to: usize) {
        self.monkeys[to].catch(item);
    }

    fn do_round(&mut self) {
        for m in 0..self.monkeys.len() {
            while let Some(worry) = self.monkeys[m].items.pop_front() {
                let (monkey_idx, worry) = self.monkeys[m].inspect(worry);
                self.throw(worry, monkey_idx);
            }
        }
    }

    pub fn do_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.do_round();
        }
    }

    pub fn get_inspection_counts(&self) -> Vec<u128> {
        let mut counts: Vec<u128> = self.monkeys.iter().map(|m| m.inspection_count).collect();

        counts.sort();

        counts.into_iter().rev().collect()
    }
}

fn main() {
    let file_str = include_str!("input.txt");
    let mut game = MonkeyGame::from_file_str(file_str, WorryType::Normal);
    game.do_rounds(20);

    let inspections = game.get_inspection_counts();
    let monkey_business = inspections[0] * inspections[1];

    println!("Part 1 monkey business: {}", monkey_business);

    // let mut game = MonkeyGame::from_file_str(file_str, WorryType::Extra);
    // game.do_rounds(10_000);

    // let inspections = game.get_inspection_counts();
    // let monkey_business = inspections[0] * inspections[1];

    // println!("Part 2 monkey business: {}", monkey_business);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        include_str!("test-input.txt")
    }

    #[test]
    fn test_monkey_round() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Normal);
        game.do_round();

        assert_eq!(game.monkeys[0].items, VecDeque::from([20, 23, 27, 26]));
        assert_eq!(
            game.monkeys[1].items,
            VecDeque::from([2080, 25, 167, 207, 401, 1046])
        );
        assert_eq!(game.monkeys[2].items, VecDeque::new());
        assert_eq!(game.monkeys[3].items, VecDeque::new());
    }

    #[test]
    fn test_monkey_20_rounds() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Normal);
        game.do_rounds(20);

        assert_eq!(game.monkeys[0].inspection_count, 101);
        assert_eq!(game.monkeys[3].inspection_count, 105);
    }
}
