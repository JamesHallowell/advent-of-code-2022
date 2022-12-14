use std::{collections::HashMap, ops::Add};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add<(i32, i32)> for Pos {
    type Output = Self;

    fn add(self, (x, y): (i32, i32)) -> Self::Output {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Block {
    Rock,
    Sand,
    Floor,
}

#[derive(Clone)]
struct Cave {
    blocks: HashMap<Pos, Block>,
    floor: i32,
}

impl Cave {
    fn new(rocks: impl Iterator<Item = Pos>) -> Self {
        let blocks: HashMap<_, _> = rocks.map(|pos| (pos, Block::Rock)).collect();
        let floor = blocks.keys().map(|pos| pos.y + 2).max().unwrap_or(0);
        Self { blocks, floor }
    }

    fn get(&self, pos: Pos) -> Option<Block> {
        if pos.y >= self.floor {
            return Some(Block::Floor);
        }
        self.blocks.get(&pos).copied()
    }

    fn count_sand(&self) -> usize {
        self.blocks
            .values()
            .filter(|block| matches!(block, Block::Sand))
            .count()
    }

    fn spawn_sand(&mut self, mut sand: Pos) -> Pos {
        loop {
            match [(0, 1), (-1, 1), (1, 1)]
                .into_iter()
                .map(|offset| sand + offset)
                .find(|&sand| self.get(sand).is_none())
            {
                Some(pos) => sand = pos,
                None => {
                    self.blocks.insert(sand, Block::Sand);
                    return sand;
                }
            }
        }
    }
}

fn parse_rocks(input: &str) -> impl Iterator<Item = Pos> + '_ {
    input.lines().flat_map(|line| {
        let mut corners = line.split(" -> ").map(|line| {
            line.split_once(',')
                .map(|(x, y)| Pos {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                })
                .unwrap()
        });

        let mut current = corners.next();
        let mut destination = corners.next();
        std::iter::from_fn(move || -> Option<Pos> {
            let next = current;
            current = match (current, destination) {
                (Some(src), Some(dst)) if src.x == dst.x && src.y < dst.y => Some(src + (0, 1)),
                (Some(src), Some(dst)) if src.x == dst.x && src.y > dst.y => Some(src + (0, -1)),
                (Some(src), Some(dst)) if src.y == dst.y && src.x < dst.x => Some(src + (1, 0)),
                (Some(src), Some(dst)) if src.y == dst.y && src.x > dst.x => Some(src + (-1, 0)),
                (Some(src), Some(dst)) if src == dst => Some(src),
                _ => None,
            };
            if current == destination {
                destination = corners.next();
            }
            next
        })
    })
}

fn part_1(mut cave: Cave) -> usize {
    loop {
        let sand = cave.spawn_sand(Pos { x: 500, y: 0 });

        let resting_on = cave.get(sand + (0, 1));
        if matches!(resting_on, Some(Block::Floor)) {
            return cave.count_sand() - 1;
        }
    }
}

fn part_2(mut cave: Cave) -> usize {
    loop {
        if cave.get(Pos { x: 500, y: 0 }).is_some() {
            return cave.count_sand();
        }
        cave.spawn_sand(Pos { x: 500, y: 0 });
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let cave = Cave::new(parse_rocks(INPUT));
    println!("Part 1: {}", part_1(cave));

    let cave = Cave::new(parse_rocks(INPUT));
    println!("Part 2: {}", part_2(cave));
}
