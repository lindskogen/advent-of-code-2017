use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_sum(input: &str, index_delta: usize) -> u32 {
    let chars = input.chars();
    let nbrs: Vec<u32> = chars.map(|c| c.to_digit(10).unwrap()).collect();

    let mut sum: u32 = 0;

    for n in 0..nbrs.len() {
        if nbrs[(n + index_delta) % nbrs.len()] == nbrs[n] {
            sum += nbrs[n];
        }
    }

    sum
}

#[test]
fn main() {
    assert_eq!(6, calc_sum("1212", 2));
    assert_eq!(0, calc_sum("1221", 2));
    assert_eq!(4, calc_sum("123425", 3));
    assert_eq!(12, calc_sum("123123", 3));
    assert_eq!(4, calc_sum("12131415", 4));

    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let sum = calc_sum(&line[..], line.len() / 2);
        println!("Sum for line: {:?}", sum);
    }


}
