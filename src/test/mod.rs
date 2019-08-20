use crate::*;
use std::collections::BTreeMap;

const BASIC: &str = include_str!("basic.yml");

#[test]
fn it_works() {
    let val: Document = serde_yaml::from_str(BASIC).unwrap();
    let map: BTreeMap<Document, Document> = val["somedict"].as_map().unwrap();
    for (k, v) in map.iter() {
        println!("Parsing {}:{}", k, v);
        Templar::parse_str(&v.as_string().unwrap()).unwrap();
    }
}
