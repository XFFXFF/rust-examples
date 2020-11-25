use serde::{Deserialize, Serialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
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

fn ron_example(m: &Move) {
    let pretty = PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let serialized = to_string_pretty(m, pretty).expect("Serialization failed");
    let vec = serialized.as_bytes();
    let s = String::from_utf8(vec.to_owned()).unwrap();
    let deserialized: Move = from_str(&s).expect("Deserialization failed");
    println!("{:?}", deserialized);
}

fn main() {
    let m = Move {
        direction: Direction::East,
        step: 5
    };
    json_example(&m);
    ron_example(&m);
}