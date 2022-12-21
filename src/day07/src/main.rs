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
            name,
            weight,
            names_above,
        }
    }
}

fn main() {
    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let programs: HashMap<String, Program> = f.lines()
        .map(|line| {
            let p = Program::parse(&line.expect("Unable to read line")[..]);
            (p.name.clone(), p)
        })
        .collect();

    let mut map: HashMap<&String, &Program>  = HashMap::new();


    for (_, p) in programs.iter() {
        for name in p.names_above.iter() {
            map.insert(name, p);
        }
    }

    if let Some(start_program) = programs.values().collect::<Vec<_>>().first() {

        let mut root = start_program;

        while let Some(program) = map.get(&root.name) {
            root = &program;
        }

        println!("Part 1: {}", root.name);

        let mut program_name = &root.name;
        let mut diff = 0;

        while let Some(program) = programs.get(program_name) {
            let mut siblings = program.names_above.iter().enumerate()
                .map(|(index, n)|
                    (index, sum_sub_tower(&programs, programs.get(n).expect("no program")))
                ).collect::<Vec<_>>();
            siblings.sort_by_key(|a| a.1);

            let first = siblings.get(0).expect("no first");
            let middle = siblings.get(siblings.len() / 2usize).expect("no middle");
            let last = siblings.last().expect("no last");

            if first.1 != last.1 {
                if first.1 == middle.1 {
                    diff = last.1 - middle.1;
                    program_name = program.names_above.get(last.0).unwrap();
                } else {
                    diff = middle.1 - first.1;
                    program_name = program.names_above.get(first.0).unwrap();
                }
            } else {
                println!("Part 2: {}", program.weight - diff);
                return;
            }
        }
    }
}

fn sum_sub_tower(programs: &HashMap<String, Program>, root: &Program) -> u32 {
    let weight = root.weight;

    if root.names_above.is_empty() {
        return weight;
    }

    let siblings: Vec<u32> = root.names_above.iter().map(|n| {
        programs.get(n).map_or(0, |p| sum_sub_tower(&programs,p))
    }).collect();

    return weight + siblings.iter().sum::<u32>();
}
