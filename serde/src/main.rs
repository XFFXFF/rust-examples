#[macro_use]
extern crate bson;
use serde::{Deserialize, Serialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
use bson::Document;
// use std::io::Cursor;
use std::io::Write;
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
    #[serde(with = "bson::compat::u2f")]
    step: u32,
}

fn json_example(m: &Move) {
    let f = File::create("single_move.json").unwrap();
    serde_json::to_writer(f, m).unwrap();

    let f = File::open("single_move.json").unwrap();
    let reader = BufReader::new(f);
    let deserialized: Move = serde_json::from_reader(reader).unwrap();

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

fn bson_example(m: &Move) {
    let serialized = bson::to_bson(&m).unwrap();
    let f = File::create("1000_move.bson").unwrap();
    {
        let mut writer = BufWriter::new(f);
        for _ in (0..1000).rev() {
            serialized.as_document().unwrap().to_writer(&mut writer).unwrap();
        }
    }

    let mut f = File::open("1000_move.bson").unwrap();
    while let Ok(deserialized) = Document::from_reader(&mut f) {
        println!("{:?}", deserialized);
    }
}

fn main() {
    let m = Move {
        direction: Direction::East,
        step: 5
    };
    json_example(&m);
    ron_example(&m);
    bson_example(&m);
}