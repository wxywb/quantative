mod engine;
use engine::Engine;
mod strategy;
use std::collections::HashMap;


fn main () {
    let engine = Engine{name: "hello".to_string(), strategies: HashMap::new()};
    println!("hello world {} ", engine.name);
}