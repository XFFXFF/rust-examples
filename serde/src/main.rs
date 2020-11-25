use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    direction: Direction,
    step: u32,
}

fn json_example(m: &Move) {
    let serialized = serde_json::to_string(&m).unwrap();
    let f = File::create("move.txt").unwrap();
    {
        let mut writer = BufWriter::new(f);
        writer.write(serialized.as_bytes()).unwrap();
    }

    let f = File::open("move.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();

    let deserialized: Move = serde_json::from_str(&buffer).unwrap();
    println!("{:?}", deserialized);
}

fn main() {
    let m = Move {
        direction: Direction::East,
        step: 5
    };
    json_example(&m);
}