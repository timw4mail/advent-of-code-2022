use malachite::integer::Integer;
use malachite::num::arithmetic::traits::DivisibleBy;
use malachite::num::arithmetic::traits::Square;
use malachite::num::basic::traits::Zero;
use std::collections::VecDeque;
use std::str::FromStr;

/// Just to cut down on boilerplate for operations
/// with primitive types
fn big(n: usize) -> Integer {
    n.into()
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WorryType {
    Normal,
    Extra,
}

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Old,
    Literal(Integer)
}

#[derive(Debug)]
struct Operation {
    operator: char,
    operand: Operand,
}

impl Operation {
    fn new(operator: char, operand: &str) -> Self {
        let operand = match operand {
            "old" => Operand::Old,
            _ => Operand::Literal(Integer::from_str(operand).unwrap()),
        };

        Operation {
            operator,
            operand,
        }
    }

    #[inline(always)]
    fn run(&self, old: Integer) -> Integer {
        let operand = self.operand.clone();
        if operand == Operand::Old && self.operator == '*' {
            return old.square();
        }

        let other = match operand {
            Operand::Old => old.clone(),
            Operand::Literal(other) => other,
        };

        match self.operator {
            '+' => old + other,
            '*' => old * other,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<Integer>,
    operation: Operation,
    test: Integer,
    pass_monkey: usize,
    fail_monkey: usize,
    inspection_count: Integer,
    inspection_worry: WorryType,
}

impl Monkey {
    pub fn from_behavior(raw: &str, inspection_worry: WorryType) -> Self {
        let lines: Vec<&str> = raw.lines().collect();

        let item_parts: Vec<&str> = lines[1].split(": ").collect();
        let items: VecDeque<Integer> = item_parts[1]
            .split(", ")
            .map(|i| i.parse::<Integer>().unwrap())
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
            operation: Operation::new(
                operator.chars().next().unwrap(),
                operand
            ),
            test: big(test),
            pass_monkey,
            fail_monkey,
            inspection_count: Integer::ZERO,
            inspection_worry,
        }
    }

    #[inline(always)]
    fn run_test(&self, item: &Integer) -> usize {
        if item.divisible_by(&self.test) {
            self.pass_monkey
        } else {
            self.fail_monkey
        }
    }

    #[inline(always)]
    pub fn inspect(&mut self, item: Integer) -> (usize, Integer) {
        self.inspection_count += big(1);

        let worry = if self.inspection_worry == WorryType::Normal {
            self.operation.run(item) / big(3)
        } else {
            self.operation.run(item)
        };

        let new_monkey = self.run_test(&worry);

        (new_monkey, worry)
    }

    #[inline(always)]
    pub fn catch(&mut self, item: Integer) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
pub struct MonkeyGame {
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

    fn throw(&mut self, item: Integer, to: usize) {
        self.monkeys[to].catch(item);
    }

    #[inline(always)]
    pub fn do_rounds(&mut self, rounds: usize) -> &Self {
        for r in 0..rounds {
            if r % 100 == 0 {
                println!("Running round {}", r);
            }

            for m in 0..self.monkeys.len() {
                while let Some(worry) = self.monkeys[m].items.pop_front() {
                    let (monkey_idx, worry) = self.monkeys[m].inspect(worry);
                    self.throw(worry, monkey_idx);
                }
            }
        }

        self
    }

    pub fn get_inspection_counts(&self) -> Vec<Integer> {
        let mut counts: Vec<Integer> = self
            .monkeys
            .iter()
            .map(|m| m.inspection_count.clone())
            .collect();

        counts.sort();

        counts.into_iter().rev().collect()
    }

    pub fn get_monkey_business(&self) -> Integer {
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

    let monkey_business2 = MonkeyGame::from_file_str(file_str, WorryType::Extra)
        .do_rounds(10_000)
        .get_monkey_business();
    println!("Part 2 monkey business: {}", monkey_business2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        include_str!("test-input.txt")
    }

    #[test]
    fn monkey_round() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Normal);
        game.do_rounds(1);

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
    fn monkey_20_rounds() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Normal);
        game.do_rounds(20);

        assert_eq!(game.monkeys[0].inspection_count, big(101));
        assert_eq!(game.monkeys[3].inspection_count, big(105));
        assert_eq!(game.get_monkey_business(), big(10605));
    }

    fn monkey_10_000_rounds() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Normal);
        game.do_rounds(10_000);

        assert_eq!(game.monkeys[0].inspection_count, big(52166));
        assert_eq!(game.monkeys[3].inspection_count, big(52013));
    }

    #[test]
    fn monkey_20_rounds_extra_worry() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Extra);
        game.do_rounds(20);

        assert_eq!(game.monkeys[0].inspection_count, big(99));
        assert_eq!(game.monkeys[3].inspection_count, big(103));
        assert_eq!(game.get_monkey_business(), big(10197));
    }


    fn monkey_1000_rounds_extra_worry() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Extra);
        game.do_rounds(1000);

        assert_eq!(game.monkeys[0].inspection_count, big(5204));
        assert_eq!(game.monkeys[3].inspection_count, big(5192));
    }


    fn monkey_10_000_rounds_extra_worry() {
        let mut game = MonkeyGame::from_file_str(get_test_data(), WorryType::Extra);
        game.do_rounds(10_000);

        assert_eq!(game.monkeys[0].inspection_count, big(52166));
        assert_eq!(game.monkeys[3].inspection_count, big(52013));
    }
}
