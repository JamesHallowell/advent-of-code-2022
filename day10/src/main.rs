enum Instruction {
    Noop,
    Add(i32),
}

struct Cpu<I> {
    instructions: I,
    register: i32,
    pending_add: Option<i32>,
}

impl<I> Cpu<I> {
    fn new(instructions: I) -> Self {
        Self {
            instructions,
            register: 1,
            pending_add: None,
        }
    }
}

impl<I> Iterator for Cpu<I>
where
    I: Iterator<Item = Instruction>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let register_during_execution = self.register;

        match self.pending_add.take() {
            Some(value) => {
                self.register += value;
            }
            None => match self.instructions.next() {
                Some(Instruction::Noop) => {}
                Some(Instruction::Add(value)) => {
                    self.pending_add = Some(value);
                }
                None => return None,
            },
        };

        Some(register_during_execution)
    }
}

fn signal_strength((cycle, register): (usize, i32)) -> i32 {
    (cycle + 1) as i32 * register
}

fn calculate_pixel((cycle, register): (usize, i32)) -> char {
    if [-1, 0, 1]
        .into_iter()
        .map(|offset| register + offset)
        .any(|sprite| sprite == (cycle % 40) as i32)
    {
        '#'
    } else {
        '.'
    }
}

fn add_new_line(row: &[char]) -> impl Iterator<Item = &char> {
    row.iter().chain(&['\n'])
}

fn main() {
    let result: i32 = Cpu::new(instructions(include_str!("input.txt")))
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(signal_strength)
        .sum();

    println!("Part 1: {result}");

    let result: String = Cpu::new(instructions(include_str!("input.txt")))
        .enumerate()
        .map(calculate_pixel)
        .collect::<Vec<_>>()
        .chunks(40)
        .flat_map(add_new_line)
        .collect();

    println!("Part 2:\n{result}");
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| match &line[..4] {
        "noop" => Instruction::Noop,
        "addx" => Instruction::Add(
            line.split_once(' ')
                .and_then(|(_, arg)| arg.parse::<i32>().ok())
                .unwrap(),
        ),
        invalid => panic!("invalid instruction '{invalid}'"),
    })
}
