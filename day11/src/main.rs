use num_bigint::BigUint;
use std::collections::VecDeque;
use std::ops::*;
use std::str::FromStr;

/// Just to cut down on boilerplate for operations
/// with primitive types
#[inline(always)]
fn big(n: usize) -> BigUint {
    n.into()
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WorryType {
    Normal,
    Extra,
}

#[derive(Debug)]
pub struct Operation {
    pub operator: char,
    pub operand: String,
}

impl Operation {
    #[inline(always)]
    pub fn run(&self, old: BigUint) -> BigUint {
        let other = match self.operand.as_str() {
            "old" => old.clone(),
            _ => BigUint::from_str(&self.operand).unwrap(),
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
pub struct Monkey {
    pub items: VecDeque<BigUint>,
    pub operation: Operation,
    pub test: usize,
    pub pass_monkey: usize,
    pub fail_monkey: usize,
    pub inspection_count: BigUint,
    pub inspection_worry: WorryType,
}

impl Monkey {
    pub fn from_behavior(raw: &str, inspection_worry: WorryType) -> Self {
        let lines: Vec<&str> = raw.lines().collect();

        let item_parts: Vec<&str> = lines[1].split(": ").collect();
        let items: VecDeque<BigUint> = item_parts[1]
            .split(", ")
            .map(|i| i.parse::<BigUint>().unwrap())
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
            inspection_count: big(0),
            inspection_worry,
        }
    }

    pub fn run_test(&self, item: BigUint) -> usize {
        if item % big(self.test) == big(0) {
            self.pass_monkey
        } else {
            self.fail_monkey
        }
    }

    #[inline(always)]
    pub fn inspect(&mut self, item: BigUint) -> (usize, BigUint) {
        self.inspection_count += big(1);

        let worry = if self.inspection_worry == WorryType::Normal {
            self.operation.run(item) / big(3)
        } else {
            self.operation.run(item)
        };

        let new_monkey = self.run_test(worry.clone());

        (new_monkey, worry)
    }

    pub fn catch(&mut self, item: BigUint) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
pub struct MonkeyGame {
    pub monkeys: Vec<Monkey>,
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

    pub fn throw(&mut self, item: BigUint, to: usize) {
        self.monkeys[to].catch(item);
    }

    pub fn do_round(&mut self) {
        for m in 0..self.monkeys.len() {
            while let Some(worry) = self.monkeys[m].items.pop_front() {
                let (monkey_idx, worry) = self.monkeys[m].inspect(worry);
                self.throw(worry, monkey_idx);
            }
        }
    }

    pub fn do_rounds(&mut self, rounds: usize) -> &Self {
        for r in 0..rounds {
            if r % 100 == 0 {
                println!("Running round {}", r);
            }

            self.do_round();
        }

        self
    }

    pub fn get_inspection_counts(&self) -> Vec<BigUint> {
        let mut counts: Vec<BigUint> = self
            .monkeys
            .iter()
            .map(|m| m.inspection_count.clone())
            .collect();

        counts.sort();

        counts.into_iter().rev().collect()
    }

    pub fn get_monkey_business(&self) -> BigUint {
        let inspections = self.get_inspection_counts();

        inspections.get(0).unwrap() * inspections.get(1).unwrap()
    }
}

fn main() {
    let file_str = include_str!("input.txt");
    let monkey_business1 = MonkeyGame::from_file_str(file_str, WorryType::Normal)
        .do_rounds(20)
        .get_monkey_business();

    println!("Part 1 monkey business: {}", monkey_business1);

    let monkey_business1 = MonkeyGame::from_file_str(file_str, WorryType::Extra)
        .do_rounds(500)
        .get_monkey_business();

    println!("monkey business 400 rounds: {}", monkey_business1);

    // let monkey_business2 = MonkeyGame::from_file_str(file_str, WorryType::Extra)
    //     .do_rounds(10_000)
    //     .get_monkey_business();
    //
    // println!("Part 2 monkey business: {}", monkey_business2);
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

        assert_eq!(
            game.monkeys[0].items,
            VecDeque::from([big(20), big(23), big(27), big(26)])
        );
        assert_eq!(
            game.monkeys[1].items,
            VecDeque::from([
                big(2080),
                big(25),
                big(167),
                big(207),
                big(401),
                big(1046)
            ])
        );
        assert_eq!(game.monkeys[2].items, VecDeque::new());
        assert_eq!(game.monkeys[3].items, VecDeque::new());
    }

    #[test]
    fn test_monkey_20_rounds() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Normal);
        game.do_rounds(20);

        assert_eq!(game.monkeys[0].inspection_count, big(101));
        assert_eq!(game.monkeys[3].inspection_count, big(105));
    }

    fn test_monkey_10000_rounds() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Extra);
        game.do_rounds(10_000);

        assert_eq!(game.monkeys[0].inspection_count, big(52166));
        assert_eq!(game.monkeys[3].inspection_count, big(52013));
    }
}
