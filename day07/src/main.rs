use std::collections::HashMap;

fn directory_sizes(input: &str) -> HashMap<String, usize> {
    #[derive(Debug)]
    struct State<'input> {
        cd: Vec<&'input str>,
        sizes: HashMap<String, usize>,
    }

    input
        .lines()
        .fold(
            State {
                cd: vec!["~"],
                sizes: HashMap::default(),
            },
            |mut state, line| {
                let tokens = line.split_whitespace().collect::<Vec<_>>();
                match tokens.as_slice() {
                    ["$", "cd", "/"] => state.cd = vec!["~"],
                    ["$", "cd", ".."] => {
                        state.cd.pop();
                    }
                    ["$", "cd", dir] => state.cd.push(dir),
                    ["$", "ls"] => {}
                    ["dir", _] => {}
                    [size, _] => {
                        let size = size.parse().unwrap();

                        for i in 0..state.cd.len() {
                            state
                                .sizes
                                .entry(state.cd[..=i].join("/"))
                                .and_modify(|total| *total += size)
                                .or_insert(size);
                        }
                    }
                    _ => panic!("unexpected"),
                };
                state
            },
        )
        .sizes
}

fn part_1(input: &str) -> usize {
    directory_sizes(input)
        .values()
        .filter(|&size| *size <= 100_000)
        .sum()
}

fn part_2(input: &str) -> usize {
    let sizes = directory_sizes(input);

    const TOTAL: usize = 70_000_000;
    const REQUIRED: usize = 30_000_000;
    let need_to_delete = REQUIRED - (TOTAL - sizes["~"]);

    sizes
        .values()
        .filter(|&size| *size >= need_to_delete)
        .min()
        .copied()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("input.txt")));
    println!("Part 2: {}", part_2(include_str!("input.txt")));
}
