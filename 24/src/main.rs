use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

type Bridge = Vec<BridgePart>;

#[derive(Eq, PartialEq, Hash, Clone)]
struct BridgePart {
    input: u32,
    output: u32,
}

impl fmt::Debug for BridgePart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.input, self.output)
    }
}

impl BridgePart {
    fn parse(string: &String) -> BridgePart {
        let parts: Vec<u32> = string
            .split('/')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        BridgePart {
            input: parts[0],
            output: parts[1],
        }
    }

    fn has_port(&self, port: u32) -> bool {
        self.input == port || self.output == port
    }

    fn other_port(&self, port: u32) -> u32 {
        if self.input != port {
            self.input
        } else {
            self.output
        }
    }

    fn value(&self) -> u32 {
        self.input + self.output
    }
}

fn bridge_sum(bridge: &Bridge) -> u32 {
    bridge.iter().map(BridgePart::value).sum()
}

fn make_bridge(start: &Bridge, open_port: u32, parts: HashSet<BridgePart>) -> Vec<Bridge> {

    let compatible_parts: Bridge = parts
        .iter()
        .filter(|part| part.has_port(open_port))
        .cloned()
        .collect();
    let length = compatible_parts.len();

    if length == 0 {
        vec![start.clone()]
    } else {
        compatible_parts
            .iter()
            .flat_map(|part| {
                let other_port = part.other_port(open_port);
                let mut rest_parts = parts.clone();
                rest_parts.remove(part);
                let mut bridge = start.clone();
                bridge.push(part.clone());

                make_bridge(&bridge, other_port, rest_parts)
            })
            .collect()
    }
}

fn max_value_bridges(bridges: &Vec<Bridge>) -> u32 {
    bridges.iter().map(bridge_sum).max().unwrap()
}

fn main() {
    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let parts: HashSet<BridgePart> = f.lines()
        .map(|l| BridgePart::parse(&l.expect("Unable to read line")))
        .collect();

    let bridges: Vec<Bridge> = make_bridge(&vec![], 0, parts);
    println!("Num bridges considered: {}", bridges.len());

    let max_value = max_value_bridges(&bridges);

    let max_len = bridges.iter().map(Vec::len).max().unwrap();

    let max_len_bridges: Vec<Bridge> = bridges
        .iter()
        .filter(|b| b.len() == max_len)
        .cloned()
        .collect();

    let max_value_for_max_length = max_value_bridges(&max_len_bridges);

    println!("Max value for any bridge {:?}", max_value);
    println!(
        "Max value for max length bridge {:?}",
        max_value_for_max_length
    );

}
