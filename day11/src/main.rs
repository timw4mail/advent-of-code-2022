use std::collections::VecDeque;
use std::ops::*;

#[derive(Debug)]
struct Operation {
    operator: char,
    operand: String,
}

impl Operation {
    fn run(&self, old: usize) -> usize {
        let other = match self.operand.as_str() {
            "old" => old,
            _ => self.operand.parse::<usize>().unwrap(),
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
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    pass_monkey: usize,
    fail_monkey: usize,
    inspection_count: usize,
}

impl Monkey {
    fn from_behavior(raw: &str) -> Self {
        let lines: Vec<&str> = raw.lines().collect();

        let item_parts: Vec<&str> = lines[1].split(": ").collect();
        let items: VecDeque<usize> = item_parts[1]
            .split(", ")
            .map(|i| i.parse::<usize>().unwrap())
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
        }
    }

    fn run_test(&self, item: usize) -> usize {
        if item % self.test == 0 {
            self.pass_monkey
        } else {
            self.fail_monkey
        }
    }

    pub fn inspect(&mut self, item: usize) -> (usize, usize) {
        self.inspection_count += 1;

        let worry = self.operation.run(item);
        let worry = worry / 3;

        let new_monkey = self.run_test(worry);

        (new_monkey, worry)
    }

    pub fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
struct MonkeyGame {
    monkeys: Vec<Monkey>,
}

impl MonkeyGame {
    pub fn from_file_str(file_str: &'static str) -> Self {
        let behaviors = file_str.split("\n\n");

        Self {
            monkeys: behaviors.map(Monkey::from_behavior).collect(),
        }
    }

    fn throw(&mut self, item: usize, to: usize) {
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
}

fn main() {
    let file_str = include_str!("input.txt");
    let mut game = MonkeyGame::from_file_str(file_str);
    game.do_round();

    println!("{:#?}", game);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        include_str!("test-input.txt")
    }

    #[test]
    fn test_monkey_round() {
        let mut game = MonkeyGame::from_file_str(get_test_data());
        game.do_round();

        assert_eq!(game.monkeys[0].items, VecDeque::from([20, 23, 27, 26]));
        assert_eq!(
            game.monkeys[1].items,
            VecDeque::from([2080, 25, 167, 207, 401, 1046])
        );
        assert_eq!(game.monkeys[2].items, VecDeque::new());
        assert_eq!(game.monkeys[3].items, VecDeque::new());
    }
}
