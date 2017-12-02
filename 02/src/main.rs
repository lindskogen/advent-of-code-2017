use std::fs::File;
use std::io::{BufRead, BufReader};

fn checksum(input: &str) -> u32 {
    let strings = input.split_whitespace();
    let nbrs: Vec<u32> = strings.map(|c| c.parse::<u32>().unwrap()).collect();

    let mut min = u32::max_value();
    let mut max = u32::min_value();

    for n in nbrs {
        if n > max {
            max = n;
        }
        if n < min {
            min = n;
        }
    }

    max - min
}

fn main() {
    assert_eq!(8, checksum("5 1 9 5"));
    assert_eq!(4, checksum("7 5 3"));
    assert_eq!(6, checksum("2 4 6 8"));

    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let res: u32 = f.lines()
        .map(|line| checksum(&line.expect("Error reading line")[..]))
        .sum();
    println!("Checksum for file: {:?}", res);

}
