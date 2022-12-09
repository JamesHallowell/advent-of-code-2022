use std::{collections::HashSet, iter::repeat};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Knot {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

fn are_knots_touching(a: Knot, b: Knot) -> bool {
    a.x.abs_diff(b.x) <= 1 && a.y.abs_diff(b.y) <= 1
}

fn move_head(head: Knot, motion: Motion) -> Knot {
    match motion {
        Motion::Up => Knot {
            y: head.y + 1,
            ..head
        },
        Motion::Down => Knot {
            y: head.y - 1,
            ..head
        },
        Motion::Left => Knot {
            x: head.x - 1,
            ..head
        },
        Motion::Right => Knot {
            x: head.x + 1,
            ..head
        },
    }
}

fn move_tail((head, tail): (Knot, Knot)) -> Knot {
    if are_knots_touching(head, tail) {
        return tail;
    }

    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .filter(|&(x, y)| {
        if head.x == tail.x || head.y == tail.y {
            x == 0 || y == 0
        } else {
            x != 0 && y != 0
        }
    })
    .map(|(x, y)| Knot {
        x: tail.x + x,
        y: tail.y + y,
    })
    .find(|&knot| are_knots_touching(head, knot))
    .unwrap()
}

fn parse_motions(input: &str) -> impl Iterator<Item = Motion> + '_ {
    input
        .lines()
        .map(|line| {
            let (motion, n) = line.split_once(' ').unwrap();
            let motion = match motion {
                "U" => Motion::Up,
                "D" => Motion::Down,
                "L" => Motion::Left,
                "R" => Motion::Right,
                _ => panic!("unexpected"),
            };
            let n: usize = n.parse().unwrap();
            (motion, n)
        })
        .flat_map(|(motion, n)| repeat(motion).take(n))
}

fn count_unique_tail_positions<const N: usize>(motions: impl Iterator<Item = Motion>) -> usize {
    let rope = [Knot::default(); N];
    let (_, tail_positions) = motions.fold(
        (rope, HashSet::new()),
        |(mut rope, mut tail_positions), motion| {
            rope[0] = move_head(rope[0], motion);
            for i in 1..rope.len() {
                rope[i] = move_tail((rope[i - 1], rope[i]));
            }
            tail_positions.insert(rope[rope.len() - 1]);
            (rope, tail_positions)
        },
    );
    tail_positions.len()
}

fn main() {
    println!(
        "Part 1: {}",
        count_unique_tail_positions::<2>(parse_motions(include_str!("input.txt")))
    );
    println!(
        "Part 2: {}",
        count_unique_tail_positions::<10>(parse_motions(include_str!("input.txt")))
    );
}
