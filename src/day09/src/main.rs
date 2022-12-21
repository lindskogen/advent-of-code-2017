use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum ParserState {
    Initial,
    Garbage,
    GarbageIgnore,
    Ignore,
}

fn parse_stream(string: &str) -> (u32, u32) {
    let group_count = 0;
    let group_score = 0;
    let parser_state = ParserState::Initial;
    let parsed_chars = 0;

    let mut state = (parser_state, group_count, group_score, parsed_chars);

    for ch in string.chars() {
        let (st, count, score, parsed) = state;

        state = match (st, ch) {
            (ParserState::Initial, '{') => (st, count + 1, score, parsed),
            (ParserState::Initial, '}') => (st, count - 1, score + count, parsed),
            (ParserState::Initial, '<') => (ParserState::Garbage, count, score, parsed),
            (ParserState::Garbage, '>') => (ParserState::Initial, count, score, parsed),

            (ParserState::Garbage, '!') => (ParserState::GarbageIgnore, count, score, parsed),
            (ParserState::Initial, '!') => (ParserState::Ignore, count, score, parsed),

            (ParserState::Initial, _) => (st, count, score, parsed),
            (ParserState::Garbage, _) => (st, count, score, parsed + 1),

            (ParserState::Ignore, _) => (ParserState::Initial, count, score, parsed + 1),
            (ParserState::GarbageIgnore, _) => (ParserState::Garbage, count, score, parsed),
        }
    }

    let (_, _, group_score, parsed_chars) = state;

    (group_score, parsed_chars)
}

fn main() {
    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let string = f.lines().nth(0).unwrap().expect("Error reading line");


    assert_eq!(parse_stream("{}").0, 1);
    assert_eq!(parse_stream("{{{}}}").0, 6);
    assert_eq!(parse_stream("{{},{}}").0, 5);
    assert_eq!(parse_stream("{{{},{},{{}}}}").0, 16);
    assert_eq!(parse_stream("{<a>,<a>,<a>,<a>}").0, 1);
    assert_eq!(parse_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
    assert_eq!(parse_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
    assert_eq!(parse_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);

    assert_eq!(parse_stream("<>").1, 0);
    assert_eq!(parse_stream("<random characters>").1, 17);
    assert_eq!(parse_stream("<<<<>").1, 3);
    assert_eq!(parse_stream("<{!>}>").1, 2);
    assert_eq!(parse_stream("<!!>").1, 0);
    assert_eq!(parse_stream("<!!!>>").1, 0);
    assert_eq!(parse_stream("<{o\"i!a,<{i<a>").1, 10);

    let (score_part1, score_part2) = parse_stream(&string);

    println!("Score part 1: {}", score_part1);
    println!("Score part 2: {}", score_part2);
}
