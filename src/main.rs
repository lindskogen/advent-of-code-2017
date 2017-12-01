use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_sum(input: &str) -> u32 {
    let chars = input.chars();
    let nbrs: Vec<u32> = chars.map(|c| c.to_digit(10).unwrap()).collect();

    let mut sum: u32 = 0;
    let index_delta = nbrs.len() / 2;

    for n in 0..nbrs.len() {
        if nbrs[(n + index_delta) % nbrs.len()] == nbrs[n] {
            sum += nbrs[n];
        }
    }

    println!("{:?}", sum);

    sum
}

fn main() {
    assert_eq!(6, calc_sum("1212"));
    assert_eq!(0, calc_sum("1221"));
    assert_eq!(4, calc_sum("123425"));
    assert_eq!(12, calc_sum("123123"));
    assert_eq!(4, calc_sum("12131415"));

    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        calc_sum(&line[..]);
    }


}
