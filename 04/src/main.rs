use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn sort_string(string: &str) -> String {
    let mut vec: Vec<char> = string.chars().collect();
    vec.sort();
    return vec.into_iter().collect();
}


fn validate_line(line: &str) -> bool {
    let mut set = HashSet::new();

    for word in line.split_whitespace() {
        let word = sort_string(word);

        if !set.insert(word) {
            return false;
        }
    }

    return true;
}

fn main() {
    assert_eq!(true, validate_line("aa bb cc dd ee"));
    assert_eq!(false, validate_line("aa bb cc dd aa"));
    assert_eq!(true, validate_line("aa bb cc dd aaa"));

    assert_eq!(true, validate_line("abcde fghij"));
    assert_eq!(false, validate_line("abcde xyz ecdab"));
    assert_eq!(true, validate_line("a ab abc abd abf abj"));
    assert_eq!(true, validate_line("iiii oiii ooii oooi oooo"));
    assert_eq!(false, validate_line("oiii ioii iioi iiio"));

    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let res: Vec<bool> = f.lines()
        .map(|line| validate_line(&line.expect("Error reading line")[..]))
        .filter(|x| *x)
        .collect();

    println!("{}", res.len());
}
