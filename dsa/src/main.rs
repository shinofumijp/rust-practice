use dsa::hash::HashMap;
use dsa::hash::HashSet;

use dsa::Config;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.query == "hash_set" {
        test_hash_set();
    } else if config.query == "hash_map" {
        test_hash_map();
    } else {
        println!("Nothing matched");
    }
}

fn test_hash_set() {
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(5);
    set.insert(10);
    set.insert(12);
    println!("set.contains(10)={}", set.contains(10));
}

fn test_hash_map() {
    let mut map: HashMap = HashMap::new();
    map.insert(10, 100);
    map.insert(11, 20);

    match map.get(10) {
        Some(x) => println!("map[10]={x}"),
        _ => println!("map[10]=null"),
    }
    match map.get(12) {
        Some(x) => println!("map[10]={x}"),
        _ => println!("map[10]=null"),
    }
}
