use itertools::Itertools;
use std::{collections::HashSet, iter::Sum};

fn main() {
    let result: Priority = rucksacks()
        .fold(Vec::new(), |mut items, rucksack| {
            items.extend(rucksack.common_items_in_both_compartments().into_iter());
            items
        })
        .into_iter()
        .map(Item::priority)
        .sum();

    println!("Part 1: {:?}", result);

    let result: Priority = groups()
        .map(|group| group.badge())
        .map(Item::priority)
        .sum();

    println!("Part 2: {:?}", result);
}

fn rucksacks() -> impl Iterator<Item = Rucksack> {
    include_str!("input.txt").lines().map(Rucksack::new)
}

fn groups() -> impl Iterator<Item = Group> {
    rucksacks()
        .chunks(3)
        .into_iter()
        .map(|chunk| Group(chunk.collect::<Vec<_>>().try_into().unwrap()))
        .collect::<Vec<_>>()
        .into_iter()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Item(char);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Priority(i32);

#[derive(Debug, Clone)]
struct Rucksack(Vec<Item>);

#[derive(Debug, Clone)]
struct Group([Rucksack; 3]);

impl Item {
    fn new(item: char) -> Self {
        assert!(item.is_ascii_alphabetic());
        Self(item)
    }

    fn priority(self) -> Priority {
        match self {
            Item('a'..='z') => Priority(self.0 as i32 - 'a' as i32 + 1),
            Item('A'..='Z') => Priority(self.0 as i32 - 'A' as i32 + 27),
            _ => {
                panic!("unexpected item")
            }
        }
    }
}

impl Sum<Priority> for Priority {
    fn sum<I: Iterator<Item = Priority>>(iter: I) -> Self {
        Priority(iter.fold(0_i32, |total, priority| total + priority.0))
    }
}

impl Rucksack {
    fn new(items: impl Into<String>) -> Self {
        Self(items.into().chars().map(Item::new).collect())
    }

    fn compartments(&self) -> (&[Item], &[Item]) {
        assert_eq!(self.0.len() % 2, 0);
        self.0.split_at(self.0.len() / 2)
    }

    fn common_items_in_both_compartments(&self) -> HashSet<Item> {
        let (left, right) = self.compartments();
        let left = HashSet::<Item>::from_iter(left.iter().copied());
        let right = HashSet::<Item>::from_iter(right.iter().copied());
        &left & &right
    }
}

impl IntoIterator for Rucksack {
    type Item = Item;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Group {
    fn badge(&self) -> Item {
        let mut sets = self.0.iter().cloned().map(HashSet::<Item>::from_iter);

        let intersection = sets
            .next()
            .map(|set| sets.fold(set, |set1, set2| &set1 & &set2))
            .unwrap();

        assert_eq!(intersection.len(), 1);
        intersection.into_iter().next().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn items_have_priorities() {
        assert_eq!(Item::new('a').priority(), Priority(1));
        assert_eq!(Item::new('b').priority(), Priority(2));
        assert_eq!(Item::new('c').priority(), Priority(3));

        assert_eq!(Item::new('z').priority(), Priority(26));

        assert_eq!(Item::new('A').priority(), Priority(27));
        assert_eq!(Item::new('B').priority(), Priority(28));
        assert_eq!(Item::new('C').priority(), Priority(29));

        assert_eq!(Item::new('Z').priority(), Priority(52));
    }

    #[test]
    fn split_a_rucksack_into_compartments() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        let (left, right) = rucksack.compartments();

        assert_eq!(left.len(), 12);
        assert_eq!(right.len(), 12);

        assert_eq!(left[0], Item::new('v'));
        assert_eq!(left[11], Item::new('r'));

        assert_eq!(right[0], Item::new('h'));
        assert_eq!(right[11], Item::new('p'));

        let common = rucksack.common_items_in_both_compartments();
        assert_eq!(common.len(), 1);
        assert!(common.contains(&Item::new('p')));
    }

    #[test]
    fn groups_have_a_badge() {
        let group = Group([
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::new("PmmdzqPrVvPwwTWBwg"),
        ]);

        assert_eq!(group.badge(), Item::new('r'));
    }
}
