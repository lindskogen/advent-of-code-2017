use std::fs::File;
use std::io::{BufRead, BufReader};

const PIPE: char = '|';

#[derive(Debug)]
enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

#[test]
fn it_handles_star_1_and_2() {
    let f = File::open("src/day19/input").expect("file not found");
    let f = BufReader::new(f);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut direction: Direction = Direction::SOUTH;



    let mut chars: Vec<char> = Vec::new();
    let mut steps = 0;

    let maze: Vec<Vec<char>> = f.lines()
        .map(|l| l.expect("Unable to read line").chars().collect())
        .collect();



    for (index, element) in maze[0].iter().enumerate() {
        if *element == PIPE {
            x = index;
            break;
        }
    }

    loop {
        match direction {
            Direction::SOUTH => {
                y += 1;
            }
            Direction::NORTH => {
                y -= 1;
            }
            Direction::WEST => {
                x -= 1;
            }
            Direction::EAST => {
                x += 1;
            }
        }
        steps += 1;

        match maze[y][x] {
            '|' | '-' => {}
            '+' => {
                direction = match direction {
                    Direction::SOUTH | Direction::NORTH => {
                        if maze[y][x - 1] == ' ' {
                            Direction::EAST
                        } else {
                            Direction::WEST
                        }
                    }
                    Direction::EAST | Direction::WEST => {
                        if maze[y - 1][x] == ' ' {
                            Direction::SOUTH
                        } else {
                            Direction::NORTH
                        }
                    }
                };
            }
            ' ' => {
                break;
            }
            c => {
                chars.push(c);
            }
        }
    }
    let word: String = chars.iter().collect();
    assert_eq!(word, "KGPTMEJVS");
    assert_eq!(steps, 16328);
    println!("Word: {}, steps: {}", word, steps);

}
