use std::collections::HashSet;

fn count_unique(string: &str) -> usize {
    string.chars().collect::<HashSet<_>>().len()
}

fn start_of_message(input: &str, n: usize) -> Option<usize> {
    (n..input.len()).find(|&pos| count_unique(&input[pos - n..pos]) == n)
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("Part 1: {:?}", start_of_message(INPUT, 4));
    println!("Part 2: {:?}", start_of_message(INPUT, 14));
}
