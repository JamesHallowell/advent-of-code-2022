use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    List(Vec<Value>),
    Integer(i32),
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => left.cmp(right),
            (Value::List(left), Value::List(right)) => left.cmp(right),
            (Value::Integer(left), right @ Value::List(_)) => {
                let left = Value::List(vec![Value::Integer(*left)]);
                left.cmp(right)
            }
            (left @ Value::List(_), Value::Integer(right)) => {
                let right = Value::List(vec![Value::Integer(*right)]);
                left.cmp(&right)
            }
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        value(s).map(|(value, _)| value).ok_or(())
    }
}

fn value(input: &str) -> Option<(Value, &str)> {
    if !input.starts_with('[') {
        return None;
    }

    let mut input = &input[1..];
    let mut values = Vec::new();

    loop {
        match input.chars().next() {
            Some('[') => {
                let (value, rest) = value(input)?;
                values.push(value);
                input = rest;
            }
            Some(']') => {
                input = &input[1..];
                break;
            }
            Some(',') => {
                input = &input[1..];
            }
            Some(_) => {
                let index = input.find(|c| matches!(c, ',' | ']'))?;
                let value = input[..index].parse().ok()?;
                values.push(Value::Integer(value));
                input = &input[index..];
            }
            None => {
                break;
            }
        }
    }

    Some((Value::List(values), input))
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, line)| -> Option<(usize, (Value, Value))> {
            line.split_once('\n')
                .and_then(|(left, right)| -> Option<(Value, Value)> {
                    let left = left.parse().ok()?;
                    let right = right.parse().ok()?;
                    Some((left, right))
                })
                .map(|(left, right)| (i + 1, (left, right)))
        })
        .filter_map(|(index, (left, right))| (left.cmp(&right) == Ordering::Less).then_some(index))
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut values = input
        .lines()
        .filter_map(|line| line.parse::<Value>().ok())
        .chain(["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()])
        .collect::<Vec<_>>();

    values.sort();

    let first_divider = values
        .iter()
        .position(|value| value == &"[[2]]".parse().unwrap())
        .map(|index| index + 1);

    let second_divider = values
        .iter()
        .position(|value| value == &"[[6]]".parse().unwrap())
        .map(|index| index + 1);

    first_divider
        .zip(second_divider)
        .map(|(first, second)| first * second)
        .unwrap_or_default()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}
