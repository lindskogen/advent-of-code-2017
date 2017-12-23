use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_register_value(var: &str, variables: &HashMap<String, i64>) -> i64 {
    match var.parse::<i64>() {
        Ok(num) => num,
        _ => *variables.get(var).unwrap_or(&0),
    }
}

fn main() {
    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let mut pc: usize = 0;

    let mut last_sound: i64 = 0;
    let mut last_recovered: i64 = 0;

    let mut variables: HashMap<String, i64> = HashMap::new();

    let program: Vec<String> = f.lines().map(|l| l.expect("Unable to read line")).collect();

    while pc < program.len() {
        let expr: Vec<String> = program[pc].split_whitespace().map(String::from).collect();

        match &expr[0][..] {
            "snd" => {
                last_sound = get_register_value(&expr[1][..], &variables);
            }
            "set" => {
                let value = get_register_value(&expr[2][..], &variables);
                variables.insert(expr[1].clone(), value);
            }
            "add" => {
                let prev_value = get_register_value(&expr[1][..], &variables);
                let value = get_register_value(&expr[2][..], &variables);

                variables.insert(expr[1].clone(), prev_value + value);
            }
            "mul" => {
                let prev_value = get_register_value(&expr[1][..], &variables);
                let value = get_register_value(&expr[2][..], &variables);

                variables.insert(expr[1].clone(), prev_value * value);
            }
            "mod" => {
                let prev_value = get_register_value(&expr[1][..], &variables);
                let value = get_register_value(&expr[2][..], &variables);
                variables.insert(expr[1].clone(), prev_value % value);
            }
            "rcv" => {
                let value = get_register_value(&expr[1][..], &variables);


                if value != 0 {
                    last_recovered = last_sound;
                    break;
                }
            }
            "jgz" => {
                let condition_value = get_register_value(&expr[1][..], &variables);
                let jump_value = get_register_value(&expr[2][..], &variables);

                if condition_value > 0 {
                    pc = ((pc as i64) + jump_value) as usize;
                    continue;
                }
            }
            st => {
                panic!("{} not a valid command", st);
            }
        }
        pc += 1;
    }

    println!("recv: {}", last_recovered);

}
