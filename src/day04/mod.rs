use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn sort_string(string: &str) -> String {
    let mut vec: Vec<char> = string.chars().collect();
    vec.sort();
    vec.into_iter().collect()
}


pub fn validate_line(line: &str) -> bool {
    let mut set = HashSet::new();

    for word in line.split_whitespace() {
        let word = sort_string(word);

        if !set.insert(word) {
            return false;
        }
    }

    return true;
}

#[test]
fn it_handles_star_1_and_2() {
    assert_eq!(true, validate_line("aa bb cc dd ee"));
    assert_eq!(false, validate_line("aa bb cc dd aa"));
    assert_eq!(true, validate_line("aa bb cc dd aaa"));

    assert_eq!(true, validate_line("abcde fghij"));
    assert_eq!(false, validate_line("abcde xyz ecdab"));
    assert_eq!(true, validate_line("a ab abc abd abf abj"));
    assert_eq!(true, validate_line("iiii oiii ooii oooi oooo"));
    assert_eq!(false, validate_line("oiii ioii iioi iiio"));

    let f = File::open("src/day04/input").expect("file not found");
    let f = BufReader::new(f);

    let res = f.lines()
        .filter_map(|line| {
            line.ok().filter(|s| validate_line(s))
        })
        .count();

    assert_eq!(res, 265);
}
