mod engine;
use engine::Engine;
mod strategy;
use std::collections::HashMap;
mod gateway;
use gateway::MockGateway;


fn main () {
    let engine = Engine{name: "hello".to_string(), strategies: HashMap::new(), gateway: Box::new(MockGateway{name: "hello".to_string(), subscribed_names: vec![]})};
    println!("hello world {} ", engine.name);
}