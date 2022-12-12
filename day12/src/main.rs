use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

type Pos = (usize, usize);

struct Map(Vec<Vec<char>>);

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(|line| line.chars().collect()).collect()))
    }
}

impl Map {
    fn adjacent(&self, (x, y): Pos) -> impl Iterator<Item = Pos> {
        let width = self.0[0].len();
        let height = self.0.len();

        [
            x.checked_sub(1).map(|x| (x, y)),
            ((x + 1) < width).then_some((x + 1, y)),
            y.checked_sub(1).map(|y| (x, y)),
            ((y + 1) < height).then_some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
    }

    fn squares(&self) -> impl Iterator<Item = (Pos, char)> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| ((x, y), c)))
    }

    fn start(&self) -> Pos {
        self.squares()
            .find_map(|(pos, letter)| matches!(letter, 'S').then_some(pos))
            .expect("map should have a start")
    }

    fn lowest_points(&self) -> impl Iterator<Item = Pos> + '_ {
        self.squares()
            .filter_map(|(pos, letter)| matches!(letter, 'a' | 'S').then_some(pos))
    }

    fn at(&self, pos: Pos) -> char {
        self.0[pos.1][pos.0]
    }

    fn is_end(&self, pos: Pos) -> bool {
        self.at(pos) == 'E'
    }

    fn height(&self, pos: Pos) -> usize {
        match self.at(pos) {
            'S' => 'a' as usize,
            'E' => 'z' as usize,
            letter => letter as usize,
        }
    }
}

#[derive(Debug)]
struct State {
    pos: Pos,
    steps: usize,
}

fn search_for_highest_point(map: &Map, start: Pos) -> Option<usize> {
    let state = State {
        pos: start,
        steps: 0,
    };

    let mut to_visit = VecDeque::from([state]);
    let mut visited = HashSet::from([start]);

    while let Some(current) = to_visit.pop_front() {
        if map.is_end(current.pos) {
            return Some(current.steps);
        }

        for adjacent_pos in map.adjacent(current.pos) {
            let can_climb = map.height(adjacent_pos) <= map.height(current.pos) + 1;
            let not_visited = !visited.contains(&adjacent_pos);

            if can_climb && not_visited {
                to_visit.push_back(State {
                    pos: adjacent_pos,
                    steps: current.steps + 1,
                });
                visited.insert(adjacent_pos);
            }
        }
    }

    None
}

fn part_1(map: &Map) -> Option<usize> {
    search_for_highest_point(map, map.start())
}

fn part_2(map: &Map) -> Option<usize> {
    map.lowest_points()
        .filter_map(|point| search_for_highest_point(map, point))
        .min()
}

fn main() {
    let map: Map = include_str!("input.txt").parse().expect("failed to parse");
    println!("Part 1: {}", part_1(&map).unwrap());
    println!("Part 2: {}", part_2(&map).unwrap());
}
