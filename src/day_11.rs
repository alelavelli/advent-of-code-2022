use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs::File,
    io::Read,
    time::Instant,
};

use log::info;

use crate::ProblemPart;

pub fn solve(puzzle_input: &str, part: ProblemPart) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(puzzle_input)?;
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)?;

    let result = match part {
        ProblemPart::One => {
            info!("Start solving part 1");
            let start = Instant::now();
            let result = solve_pt1(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 1 in {duration} seconds.");
            result
        }
        ProblemPart::Two => {
            info!("Start solving part 2");
            let start = Instant::now();
            let result = solve_pt2(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 2 in {duration} seconds.");
            result
        }
    };
    info!("Problem solution is {}", result);
    Ok(())
}

struct Monkey {
    items: VecDeque<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> bool>,
    divisor: u128,
    true_branch_monkey: u128,
    false_branch_monkey: u128,
}

impl Monkey {
    fn inspect_item(&mut self, no_divide: bool) -> (u128, u128) {
        let mut item = self.items.pop_front().unwrap();
        item = (self.operation)(item);

        if !no_divide {
            item = (item as f32 / 3.0).floor() as u128;
        }

        if (self.test)(item) {
            (self.true_branch_monkey, item)
        } else {
            (self.false_branch_monkey, item)
        }
    }

    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    fn add_item(&mut self, level: u128) {
        // push back
        self.items.push_back(level);
    }

    fn normalize_worry_levels(&mut self, divisor_prod: u128) {
        for item in self.items.iter_mut() {
            *item %= divisor_prod;
        }
    }
}

fn parse_input(puzzle_input: String) -> HashMap<u128, Monkey> {
    // push items back
    let mut monkeys = HashMap::new();
    for block in puzzle_input.split("\n\n") {
        let mut lines = block.lines();
        let monkey_id = lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .replace(':', "")
            .parse::<u128>()
            .unwrap();

        let mut items: VecDeque<u128> = VecDeque::new();
        for item in lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
        {
            items.push_back(item.parse().unwrap());
        }

        let operation = parse_operation(
            lines
                .next()
                .unwrap()
                .split("Operation: new = ")
                .nth(1)
                .unwrap()
                .to_string(),
        );

        let (test, divisor) = parse_test(
            lines
                .next()
                .unwrap()
                .split("Test: ")
                .nth(1)
                .unwrap()
                .to_string(),
        );

        let true_branch_monkey = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<u128>()
            .unwrap();
        let false_branch_monkey = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<u128>()
            .unwrap();

        monkeys.insert(
            monkey_id,
            Monkey {
                items,
                operation,
                test,
                divisor,
                true_branch_monkey,
                false_branch_monkey,
            },
        );
    }
    monkeys
}

fn parse_operation(operation: String) -> Box<dyn Fn(u128) -> u128> {
    let first_term = operation.split_whitespace().next().unwrap().parse::<u128>();
    let second_term = operation
        .split_ascii_whitespace()
        .nth(2)
        .unwrap()
        .parse::<u128>();

    if operation.contains('+') {
        Box::new(move |old| {
            let first_operand = first_term.clone().unwrap_or(old);
            let second_operand = second_term.clone().unwrap_or(old);
            first_operand + second_operand
        })
    } else if operation.contains('*') {
        Box::new(move |old| {
            let first_operand = first_term.clone().unwrap_or(old);
            let second_operand = second_term.clone().unwrap_or(old);
            first_operand * second_operand
        })
    } else {
        panic!("unknown operator");
    }
}

fn parse_test(test: String) -> (Box<dyn Fn(u128) -> bool>, u128) {
    if test.contains("divisible by ") {
        let num = test
            .split("divisible by ")
            .nth(1)
            .unwrap()
            .parse::<u128>()
            .unwrap();
        (Box::new(move |old| (old % num) == 0), num)
    } else {
        panic!("unknown test");
    }
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut monkeys = parse_input(puzzle_input);
    let mut monkey_businesses: HashMap<u128, u128> = HashMap::new();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let current_monkey_id = i as u128;
            while monkeys.get(&current_monkey_id).unwrap().has_items() {
                *monkey_businesses.entry(current_monkey_id).or_insert(0) += 1;
                let (destination_monkey, level) = monkeys
                    .get_mut(&current_monkey_id)
                    .unwrap()
                    .inspect_item(false);
                monkeys
                    .get_mut(&destination_monkey)
                    .unwrap()
                    .add_item(level);
            }
        }
    }

    let mut monkey_businesses_vec = monkey_businesses.into_iter().collect::<Vec<(u128, u128)>>();
    monkey_businesses_vec.sort_by(|a, b| a.1.cmp(&b.1));
    Ok((monkey_businesses_vec.last().unwrap().1
        * monkey_businesses_vec
            .get(monkey_businesses_vec.len() - 2)
            .unwrap()
            .1)
        .to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut monkeys = parse_input(puzzle_input);
    let mut monkey_businesses: HashMap<u128, u128> = HashMap::new();

    let divisors_prod = monkeys
        .values()
        .map(|x| x.divisor)
        .reduce(|acc, x| acc * x)
        .unwrap();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let current_monkey_id = i as u128;
            while monkeys.get(&current_monkey_id).unwrap().has_items() {
                monkeys
                    .get_mut(&current_monkey_id)
                    .unwrap()
                    .normalize_worry_levels(divisors_prod);
                *monkey_businesses.entry(current_monkey_id).or_insert(0) += 1;
                let (destination_monkey, level) = monkeys
                    .get_mut(&current_monkey_id)
                    .unwrap()
                    .inspect_item(true);
                monkeys
                    .get_mut(&destination_monkey)
                    .unwrap()
                    .add_item(level);
            }
        }
    }

    let mut monkey_businesses_vec = monkey_businesses.into_iter().collect::<Vec<(u128, u128)>>();
    monkey_businesses_vec.sort_by(|a, b| a.1.cmp(&b.1));
    Ok((monkey_businesses_vec.last().unwrap().1
        * monkey_businesses_vec
            .get(monkey_businesses_vec.len() - 2)
            .unwrap()
            .1)
        .to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_11_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("10605".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_11_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("2713310158".to_string(), result);

        Ok(())
    }
}
