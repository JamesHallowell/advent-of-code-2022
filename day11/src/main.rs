#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct MonkeyId(usize);

type WorryLevel = u64;

#[derive(Debug)]
struct Monkey {
    id: MonkeyId,
    items: Vec<WorryLevel>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn throw_items<'a>(
        &'a mut self,
        mitigate_worry: impl Fn(WorryLevel) -> WorryLevel + 'a,
    ) -> impl Iterator<Item = (MonkeyId, WorryLevel)> + 'a {
        self.items
            .drain(..)
            .map(|item| self.operation.apply(item))
            .map(mitigate_worry)
            .map(|item| {
                if item % self.test.divisible_by == 0 {
                    (self.test.if_true, item)
                } else {
                    (self.test.if_false, item)
                }
            })
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Constant(WorryLevel),
}

#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    fn apply(&self, item: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(Operand::Old, Operand::Old) => item + item,
            Operation::Add(Operand::Old, Operand::Constant(value))
            | Operation::Add(Operand::Constant(value), Operand::Old) => item + value,
            Operation::Multiply(Operand::Old, Operand::Old) => item * item,
            Operation::Multiply(Operand::Old, Operand::Constant(value))
            | Operation::Multiply(Operand::Constant(value), Operand::Old) => item * value,
            Operation::Add(Operand::Constant(a), Operand::Constant(b)) => a + b,
            Operation::Multiply(Operand::Constant(a), Operand::Constant(b)) => a * b,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: WorryLevel,
    if_true: MonkeyId,
    if_false: MonkeyId,
}

fn calculate_monkey_business(
    monkeys: impl Iterator<Item = Monkey>,
    rounds: usize,
    strategy_for_mitigating_worry: impl Fn(WorryLevel) -> WorryLevel,
) -> usize {
    use std::collections::{HashMap, VecDeque};

    let mut monkeys = monkeys.collect::<VecDeque<_>>();
    let mut items_inspected = HashMap::new();

    for _ in 0..rounds {
        for _ in 0..monkeys.len() {
            let mut monkey = monkeys.pop_front().unwrap();

            items_inspected
                .entry(monkey.id)
                .and_modify(|items| *items += monkey.items.len())
                .or_insert(monkey.items.len());

            for (receiving_monkey, item) in monkey.throw_items(&strategy_for_mitigating_worry) {
                let receiving_monkey = monkeys
                    .iter_mut()
                    .find(|monkey| monkey.id == receiving_monkey)
                    .unwrap();

                receiving_monkey.items.push(item);
            }

            monkeys.push_back(monkey);
        }
    }

    let mut items = items_inspected.values().collect::<Vec<_>>();
    items.sort();
    items.into_iter().rev().take(2).product()
}

fn main() {
    let monkeys = parse_monkeys(include_str!("input.txt"));
    let monkey_business = calculate_monkey_business(monkeys, 20, |worry_level| worry_level / 3);
    println!("Part 1: {monkey_business}");

    let monkeys = parse_monkeys(include_str!("input.txt")).collect::<Vec<_>>();
    let product_of_divisors: WorryLevel = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    let monkey_business = calculate_monkey_business(monkeys.into_iter(), 10_000, |worry_level| {
        worry_level % product_of_divisors
    });
    println!("Part 2: {monkey_business}");
}

fn parse_monkeys(input: &str) -> impl Iterator<Item = Monkey> + '_ {
    input.split("\n\n").map(parse_monkey)
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines();

    let id = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|line| line.split_once(':'))
        .and_then(|(id, _)| id.parse::<usize>().ok())
        .map(MonkeyId)
        .unwrap();

    let items = lines
        .next()
        .and_then(|line| line.split_once(':'))
        .map(|(_, items)| {
            items
                .split(',')
                .map(|item| item.trim().parse::<WorryLevel>())
                .collect::<Result<Vec<WorryLevel>, _>>()
                .unwrap()
        })
        .unwrap();

    let operation = lines
        .next()
        .and_then(|line| line.split_once('='))
        .map(|(_, operation)| {
            let mut split = operation.split_whitespace();

            let parse_operand = |operand: &str| {
                if operand == "old" {
                    Operand::Old
                } else {
                    Operand::Constant(operand.parse().unwrap())
                }
            };

            let a = split.next().map(parse_operand).unwrap();
            let operation = split.next().unwrap();
            let b = split.next().map(parse_operand).unwrap();

            match operation {
                "+" => Operation::Add(a, b),
                "*" => Operation::Multiply(a, b),
                _ => panic!("unexpected operation"),
            }
        })
        .unwrap();

    let mut last_substring_in_each_line = lines.map(|line| line.split_whitespace().last().unwrap());

    let divisible_by = last_substring_in_each_line
        .next()
        .and_then(|s| s.parse::<WorryLevel>().ok())
        .unwrap();

    let if_true = last_substring_in_each_line
        .next()
        .and_then(|s| s.parse::<usize>().ok())
        .map(MonkeyId)
        .unwrap();

    let if_false = last_substring_in_each_line
        .next()
        .and_then(|s| s.parse::<usize>().ok())
        .map(MonkeyId)
        .unwrap();

    Monkey {
        id,
        items,
        operation,
        test: Test {
            divisible_by,
            if_true,
            if_false,
        },
    }
}
