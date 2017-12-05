use std::fs::File;
use std::io::{BufRead, BufReader};

fn execute_part1(instructions: Vec<i32>) -> u32 {
    let mut pc: i32 = 0;
    let mut program: Vec<i32> = instructions.clone();
    let mut steps = 0;
    let length = program.len() as i32;

    while pc > -1 && pc < length {
        let instruction = program[pc as usize];
        program[pc as usize] += 1;

        pc += instruction;
        steps += 1;
    }

    return steps;
}

fn execute_part2(instructions: Vec<i32>) -> u32 {
    let mut pc: i32 = 0;
    let mut program: Vec<i32> = instructions.clone();
    let mut steps = 0;
    let length = program.len() as i32;

    while pc > -1 && pc < length {
        let instruction = program[pc as usize];
        if instruction >= 3 {
            program[pc as usize] -= 1;
        } else {
            program[pc as usize] += 1;
        }

        pc += instruction;
        steps += 1;
    }

    return steps;
}


fn main() {
    assert_eq!(5, execute_part1(vec![0, 3, 0, 1, -3]));
    assert_eq!(10, execute_part2(vec![0, 3, 0, 1, -3]));

    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let numbers: Vec<i32> = f.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let steps = execute_part1(numbers.clone());
    println!("Part 1: {}", steps);

    let steps = execute_part2(numbers);
    println!("Part 2: {}", steps);
}
