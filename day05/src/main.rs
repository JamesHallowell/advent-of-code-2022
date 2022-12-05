fn parse_start_stacks(input: &str) -> Stacks {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip(1)
        .fold(Stacks::default(), |mut stacks, line| {
            for (stack, cr8) in line
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, cr8)| *cr8 != ' ')
            {
                match stacks.get_mut(stack) {
                    Some(stack) => stack.push(cr8),
                    None => stacks.push(vec![cr8]),
                }
            }
            stacks
        })
}

fn parse_operations(input: &str) -> impl Iterator<Item = Operation> + '_ {
    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let mut split = line.split_whitespace();
            let mut get_next =
                || -> usize { split.nth(1).map(|count| count.parse().unwrap()).unwrap() };
            Operation {
                count: get_next(),
                from: get_next() - 1,
                to: get_next() - 1,
            }
        })
}

struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

type Stacks = Vec<Vec<char>>;

trait MoveCrates {
    fn move_crates(stacks: Stacks, operation: Operation) -> Stacks;
}

struct CrateMover9000;

impl MoveCrates for CrateMover9000 {
    fn move_crates(mut stacks: Vec<Vec<char>>, operation: Operation) -> Stacks {
        let crates_on_source_stack = stacks[operation.from].len();
        let crates = stacks[operation.from]
            .drain(crates_on_source_stack - operation.count..)
            .rev()
            .collect::<Vec<_>>();
        stacks[operation.to].extend(crates);
        stacks
    }
}

struct CrateMover9001;

impl MoveCrates for CrateMover9001 {
    fn move_crates(mut stacks: Vec<Vec<char>>, operation: Operation) -> Stacks {
        let crates_on_source_stack = stacks[operation.from].len();
        let crates = stacks[operation.from]
            .drain(crates_on_source_stack - operation.count..)
            .collect::<Vec<_>>();
        stacks[operation.to].extend(crates);
        stacks
    }
}

fn get_top_crates_following_rearrangement<C>(input: &str) -> String
where
    C: MoveCrates,
{
    parse_operations(input)
        .fold(parse_start_stacks(input), C::move_crates)
        .into_iter()
        .fold(String::new(), |mut string, stack| {
            string.push(stack.last().copied().unwrap());
            string
        })
}

fn main() {
    println!(
        "Part 1: {}",
        get_top_crates_following_rearrangement::<CrateMover9000>(include_str!("input.txt"))
    );
    println!(
        "Part 2: {}",
        get_top_crates_following_rearrangement::<CrateMover9001>(include_str!("input.txt"))
    );
}
