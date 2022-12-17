use {
    parse_display::{Display, FromStr},
    std::collections::HashSet,
};

#[derive(Debug, Copy, Clone, Display, FromStr, PartialEq, Eq)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Input {
    sensor: Point,
    beacon: Point,
}

impl Input {
    fn sensor_range(&self) -> u64 {
        self.sensor.distance(&self.beacon)
    }
}

#[derive(Debug, Copy, Clone, Display, FromStr, PartialEq, Eq, Hash)]
#[display("x={x}, y={y}")]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn perimeter(&self, distance: i64) -> impl Iterator<Item = Point> + '_ {
        let start = self.x - distance;
        let end = self.x + distance;
        let above = (start..=end).map(move |x| {
            (
                x,
                match x {
                    x if x == self.x => self.y - distance,
                    x if x < self.x => self.y - distance + (self.x - x),
                    x if x > self.x => self.y - distance + (x - self.x),
                    _ => unreachable!(),
                },
            )
        });
        let below = (start + 1..end).rev().map(move |x| {
            (
                x,
                match x {
                    x if x == self.x => self.y + distance,
                    x if x < self.x => self.y + distance - (self.x - x),
                    x if x > self.x => self.y + distance - (x - self.x),
                    _ => unreachable!(),
                },
            )
        });
        above.chain(below).map(|(x, y)| Point { x, y })
    }
}

fn main() {
    let input: Vec<Input> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    const Y: i64 = 2_000_000;
    let maybe_no_beacons = input.iter().fold(HashSet::new(), |mut xs, input| {
        if let Some(offset) = input
            .sensor
            .distance(&input.beacon)
            .checked_sub(input.sensor.y.abs_diff(Y))
            .map(|intersect| intersect as i64)
        {
            for x in (input.sensor.x - offset)..=(input.sensor.x + offset) {
                xs.insert(x);
            }
        }
        xs
    });

    let known_beacons = input
        .iter()
        .filter_map(|input| (input.beacon.y == Y).then_some(input.beacon.x))
        .collect::<HashSet<_>>();

    let definitely_no_beacons = maybe_no_beacons.difference(&known_beacons).count();
    println!("Part 1: {definitely_no_beacons}");

    let points_to_consider = input
        .iter()
        .flat_map(|input| {
            let distance = input.sensor.distance(&input.beacon);
            input.sensor.perimeter((distance + 1) as i64)
        })
        .filter(|point| {
            if point.x < 0 || point.x > 4_000_000 || point.y < 0 || point.y > 4_000_000 {
                return false;
            }

            input
                .iter()
                .all(|input| input.sensor_range() < input.sensor.distance(point))
        })
        .collect::<HashSet<_>>();

    assert_eq!(points_to_consider.len(), 1);
    let hidden_beacon = points_to_consider.into_iter().next().unwrap();
    let tuning_frequency = (hidden_beacon.x * 4_000_000) + hidden_beacon.y;

    println!("Part 2: {tuning_frequency}");
}
