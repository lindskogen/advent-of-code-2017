use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

#[derive(Debug, Copy, Clone)]
enum Token {
    Var(char),
    Val(i64),
}

impl Token {
    fn from(string: &str) -> Token {
        match string.parse::<i64>() {
            Ok(num) => Token::Val(num),
            _ => Token::Var(string.chars().next().unwrap()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Expr {
    Snd(Token),
    Set(Token, Token),
    Add(Token, Token),
    Mul(Token, Token),
    Mod(Token, Token),
    Rcv(Token),
    Jgz(Token, Token),
}

fn unwrap_double(expr: &str, param1: Option<&str>, param2: Option<&str>) -> Expr {
    let t1 = Token::from(param1.unwrap());
    let t2 = Token::from(param2.unwrap());

    match expr {
        "set" => Expr::Set(t1, t2),
        "add" => Expr::Add(t1, t2),
        "mul" => Expr::Mul(t1, t2),
        "mod" => Expr::Mod(t1, t2),
        "jgz" => Expr::Jgz(t1, t2),
        _ => panic!("Cannot parse exp: {:?}", expr),
    }
}

impl Expr {
    fn from(string: &String) -> Expr {
        let mut exprs = string.split_whitespace();
        let exp = exprs.next().unwrap();
        let param1 = exprs.next();
        let param2 = exprs.next();

        match exp {
            "set" | "add" | "mul" | "mod" | "jgz" => unwrap_double(exp, param1, param2),
            "rcv" => Expr::Rcv(Token::from(param1.unwrap())),
            "snd" => Expr::Snd(Token::from(param1.unwrap())),
            _ => panic!("Cannot parse exp: {:?}", exp),
        }
    }
}



#[derive(Debug)]
pub struct Program {
    values_sent: u32,
    variables: HashMap<char, i64>,
    pc: usize,
    last_sound: i64,
}

impl Program {
    fn new(p: i64) -> Program {
        let mut map = HashMap::new();
        map.insert('p', p);

        Program {
            values_sent: 0,
            variables: map,
            pc: 0,
            last_sound: 0,
        }
    }
    fn get_register_value(&self, token: &Token) -> i64 {
        match *token {
            Token::Val(num) => num,
            Token::Var(ch) => *self.variables.get(&ch).unwrap_or(&0),
        }
    }
    fn set_register_value(&mut self, token: &Token, value: i64) {
        match *token {
            Token::Var(ch) => self.variables.insert(ch, value),
            _ => None,
        };
    }
    fn update_register_value<F>(&mut self, t1: &Token, t2: &Token, closure: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let v1 = self.get_register_value(t1);
        let v2 = self.get_register_value(t2);

        self.set_register_value(t1, closure(v1, v2));
    }
    fn run_program(&mut self, program: &Vec<Expr>) -> i64 {
        while self.pc < program.len() {
            match program[self.pc] {
                Expr::Snd(token) => {
                    self.last_sound = self.get_register_value(&token);
                }
                Expr::Set(t1, t2) => {
                    self.update_register_value(&t1, &t2, |_, b| b);
                }
                Expr::Add(t1, t2) => {
                    self.update_register_value(&t1, &t2, |a, b| a + b);
                }
                Expr::Mul(t1, t2) => {
                    self.update_register_value(&t1, &t2, |a, b| a * b);
                }
                Expr::Mod(t1, t2) => {
                    self.update_register_value(&t1, &t2, |a, b| a % b);
                }
                Expr::Rcv(token) => {
                    let value = self.get_register_value(&token);
                    if value != 0 {
                        return self.last_sound;
                    }
                }
                Expr::Jgz(t1, t2) => {
                    let condition_value = self.get_register_value(&t1);
                    let jump_value = self.get_register_value(&t2);
                    if condition_value > 0 {
                        self.pc = ((self.pc as i64) + jump_value) as usize;
                        continue;
                    }
                }
            }
            self.pc += 1;
        }

        return 0;
    }

    fn run_program_channels(
        &mut self,
        program: &Vec<Expr>,
        tx: Sender<i64>,
        rx: Receiver<i64>,
    ) -> u32 {
        while self.pc < program.len() {
            match program[self.pc] {
                Expr::Snd(token) => {
                    let val = self.get_register_value(&token);
                    tx.send(val).unwrap();
                    self.values_sent += 1;
                }
                Expr::Set(t1, t2) => {
                    self.update_register_value(&t1, &t2, |_, b| b);
                }
                Expr::Add(t1, t2) => {
                    self.update_register_value(&t1, &t2, |a, b| a + b);
                }
                Expr::Mul(t1, t2) => {
                    self.update_register_value(&t1, &t2, |a, b| a * b);
                }
                Expr::Mod(t1, t2) => {
                    self.update_register_value(&t1, &t2, |a, b| a % b);
                }
                Expr::Rcv(token) => {
                    match rx.recv_timeout(Duration::from_secs(5)) {
                        Ok(value) => self.set_register_value(&token, value),
                        _ => {
                            return self.values_sent;
                        }
                    }
                }
                Expr::Jgz(t1, t2) => {
                    let condition_value = self.get_register_value(&t1);
                    let jump_value = self.get_register_value(&t2);
                    if condition_value > 0 {
                        self.pc = ((self.pc as i64) + jump_value) as usize;
                        continue;
                    }
                }
            }
            self.pc += 1;
        }
        return 0;
    }
}


#[test]
fn it_handles_star_1_and_2() {
    let f = File::open("src/day18/input").expect("file not found");
    let f = BufReader::new(f);

    let expressions: Vec<Expr> = f.lines()
        .map(|l| Expr::from(&l.expect("Unable to read line")))
        .collect();

    let mut program = Program::new(0);
    let part1 = program.run_program(&expressions);
    println!("Recovered frequency: {}", part1);
    assert_eq!(part1, 8600);

    let exps1 = expressions.clone();
    let exps2 = expressions.clone();

    let (tx1, rx2) = channel();
    let (tx2, rx1) = channel();
    let mut program1 = Program::new(0);
    let mut program2 = Program::new(1);

    let h1 = thread::spawn(move || program1.run_program_channels(&exps1, tx1, rx1));

    let h2 = thread::spawn(move || {
        let times = program2.run_program_channels(&exps2, tx2, rx2);
        println!("Send called times: {}", times);
        assert_eq!(times, 7239);
    });

    h1.join().expect("Thread 0 failed");
    h2.join().expect("Thread 1 failed");
}
