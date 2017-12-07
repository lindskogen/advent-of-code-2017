extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: u32,
    names_above: Vec<String>,
}

impl Program {
    fn parse(line: &str) -> Program {
        let re = Regex::new(r"^(\w+) \((\d+)\)").unwrap();
        let caps = re.captures(line).unwrap();

        let name = String::from(caps.get(1).unwrap().as_str());
        let weight = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        let parts: Vec<&str> = line.split("-> ").collect();
        let names_above: Vec<String>;

        if parts.len() > 1 {
            names_above = parts[1].split(", ").map(String::from).collect();
        } else {
            names_above = vec![];
        }

        Program {
            name: name,
            weight: weight,
            names_above: names_above,
        }
    }
}

fn name(programs: Vec<Program>) -> String {

    let mut map: HashMap<&str, &Program> = programs
        .iter()
        .flat_map(|p| p.names_above.into_iter().map(|n| (&n[..], p)))
        .collect();


    String::from("hello")
}

fn main() {
    let f = File::open("input-small").expect("file not found");
    let f = BufReader::new(f);

    let programs: Vec<Program> = f.lines()
        .map(|line| {
            Program::parse(&line.expect("Unable to read line")[..])
        })
        .collect();
}
