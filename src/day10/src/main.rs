use std::fs::File;
use std::io::{BufRead, BufReader};

fn reverse_range(range: &mut Vec<u8>, index: usize, length: u8) {
    for i in 0..(length / 2) {
        let index1 = (index + (i as usize)) % range.len();
        let index2 = (index + (length as usize) - 1 - (i as usize)) % range.len();
        let tmp = range[index1];
        range[index1] = range[index2];
        range[index2] = tmp;
    }
}

fn hash(upper_bound: u8, lengths: &Vec<u8>, rounds: u32) -> Vec<u8> {
    let mut range = (0 as u32..upper_bound as u32 + 1)
        .map(|x| x as u8)
        .collect::<Vec<u8>>();
    let mut current: usize = 0;
    let mut skip_size: usize = 0;

    for _ in 0..rounds {
        for len in lengths {
            reverse_range(&mut range, current, *len);
            current = (current + (*len as usize) + skip_size) % range.len();
            skip_size += 1;
        }
    }
    range
}

fn solve1(upper_bound: u8, lengths: &Vec<u8>) -> u64 {
    hash(upper_bound, lengths, 1)
        .into_iter()
        .take(2)
        .map(|x| x as u64)
        .product()
}

fn hash_string(input: &str) -> String {
    let suffix: Vec<u8> = vec![17, 31, 73, 47, 23];

    let appendage = suffix.into_iter().map(|a| a as char);
    let input = input.chars().chain(appendage).map(|a| a as u8).collect();

    let slices = &hash(255, &input, 64)[..];

    let slices: Vec<String> = slices
        .chunks(16)
        .map(|hash| hash.iter().fold(0, |acc, x| acc ^ x))
        .map(|x| format!("{:02x}", x))
        .collect();

    slices.join("")
}


fn main() {
    assert_eq!(solve1(4, &vec![3, 4, 1, 5]), 12);

    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let string = f.lines().nth(0).unwrap().expect("Error reading line");

    let lengths = string
        .split(',')
        .map(|l| l.parse::<u8>().unwrap())
        .collect();

    let product = solve1(255, &lengths);
    assert_eq!(product, 37230);

    println!("Product: {}", product);

    assert_eq!(hash_string(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(hash_string("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(hash_string("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(hash_string("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");

    let hash = hash_string(&string[..]);
    println!("Hash: {}", hash);
}
