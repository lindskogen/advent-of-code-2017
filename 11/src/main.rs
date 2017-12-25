use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Direction {
    fn from(string: &str) -> Option<Direction> {
        match string {
            "n" => Some(Direction::North),
            "ne" => Some(Direction::NorthEast),
            "se" => Some(Direction::SouthEast),
            "s" => Some(Direction::South),
            "sw" => Some(Direction::SouthWest),
            "nw" => Some(Direction::NorthWest),
            _ => None,
        }
    }

    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::SouthEast => (1, 0),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::NorthWest => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Hex {
    q: i32,
    r: i32,
}

impl Hex {
    fn new(q: i32, r: i32) -> Hex {
        Hex { q: q, r: r }
    }
    fn to_cube(&self) -> Cube {
        let Hex { q: x, r: z } = *self;
        Cube::new(x, -x - z, z)
    }
    fn add(&self, q: i32, r: i32) -> Hex {
        Hex::new(self.q + q, self.r + r)
    }

    fn distance(&self, other: &Hex) -> i32 {
        self.to_cube().distance(&other.to_cube())
    }
}

#[derive(Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Cube {
        Cube { x: x, y: y, z: z }
    }
    fn distance(&self, other: &Cube) -> i32 {
        let Cube {
            x: x1,
            y: y1,
            z: z1,
        } = *self;
        let Cube {
            x: x2,
            y: y2,
            z: z2,
        } = *other;

        *vec![(x1 - x2).abs(), (y1 - y2).abs(), (z1 - z2).abs()]
            .iter()
            .max()
            .unwrap()
    }
}

fn main() {
    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let start_pos = Hex::new(0, 0);

    let steps: Vec<Direction> = f.lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .split(',')
        .map(|l| Direction::from(l).unwrap())
        .collect();

    let mut max_distance = 0;
    let mut pos = start_pos.add(0, 0);

    for step in steps {
        let (q, r) = step.value();

        pos = pos.add(q, r);
        let distance = pos.distance(&start_pos);

        if distance > max_distance {
            max_distance = distance;
        }
    }

    println!("End distance: {}", pos.distance(&start_pos));
    println!("Max distance: {}", max_distance);
}
