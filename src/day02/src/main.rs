use std::fs::File;
use std::io::{BufRead, BufReader};


fn sort_tuple<'a>(n1: &'a u32, n2: &'a u32) -> (&'a u32, &'a u32) {
    if n1 > n2 { (n1, n2) } else { (n2, n1) }
}

fn checksum(input: &str) -> u32 {
    let strings = input.split_whitespace();
    let nbrs: Vec<u32> = strings.map(|c| c.parse::<u32>().unwrap()).collect();

    for n in &nbrs {
        for m in &nbrs {
            let (big, small) = sort_tuple(n, m);
            if big != small && big % small == 0 {
                return big / small;
            }

        }
    }
    return 0;
}

fn main() {
    assert_eq!(4, checksum("5 9 2 8"));
    assert_eq!(3, checksum("9 4 7 3"));
    assert_eq!(2, checksum("3 8 6 5"));

    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let res: u32 = f.lines()
        .map(|line| checksum(&line.expect("Error reading line")[..]))
        .sum();
    println!("Checksum for file: {:?}", res);

}
