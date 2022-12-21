use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;
use std::collections::HashMap;

fn index_of_max(array: Vec<u32>) -> (usize, u32) {
    let mut i = 0;
    let mut max_value = 0;

    for (j, &value) in array.iter().enumerate() {
        if value > max_value {
            i = j;
            max_value = value;
        }
    }

    (i, max_value)
}

pub fn rebalance_part1(input: Vec<u32>) -> u32 {
    let mut cycles: u32 = 0;
    let mut banks = input;
    let mut previous: HashSet<Vec<u32>> = HashSet::new();
    previous.insert(banks.clone());
    loop {
        let (mut index, value) = index_of_max(banks.clone());
        banks[index] = 0;
        for _ in 0..(value) {
            index = (index + 1) % banks.len();
            banks[index] += 1;
        }
        cycles += 1;
        if !previous.insert(banks.clone()) {
            break;
        }
    }
    cycles
}

pub fn rebalance_part2(input: Vec<u32>) -> u32 {
    let mut cycles: u32 = 0;
    let mut banks = input;
    let mut previous: HashMap<Vec<u32>, u32> = HashMap::new();
    previous.insert(banks.clone(), cycles);
    loop {
        let (mut index, value) = index_of_max(banks.clone());
        banks[index] = 0;
        for _ in 0..(value) {
            index = (index + 1) % banks.len();
            banks[index] += 1;
        }
        cycles += 1;
        let banks_to_insert = banks.clone();

        if previous.contains_key(&banks_to_insert) {
            break;
        }
        previous.insert(banks_to_insert, cycles);
    }
    cycles - previous.get(&banks).unwrap()
}

#[test]
fn it_handles_star_1_and_2() {
    assert_eq!(5, rebalance_part1(vec![0, 2, 7, 0]));
    assert_eq!(4, rebalance_part2(vec![0, 2, 7, 0]));

    let f = File::open("src/day06/input").expect("file not found");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        let banks: Vec<u32> = line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let cycles = rebalance_part1(banks.clone());
        assert_eq!(cycles, 14029);

        let cycles = rebalance_part2(banks);
        assert_eq!(cycles, 2765);
    }

}
