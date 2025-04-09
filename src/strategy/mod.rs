use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::any::Any;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub symbol: String,                
    pub datetime: DateTime<Utc>,       
    pub last_price: f64,               
    pub volume: f64,                   
    pub bid_price: f64,                
    pub bid_volume: f64,               
    pub ask_price: f64,                
    pub ask_volume: f64,               
}

pub trait Strategy: Any {
    fn on_tick(&mut self, tick: &Tick) -> anyhow::Result<()>;
}

#[test] 
fn test_tick_creation() {
    let tick = Tick {
        symbol: "AAPL".to_string(),
        datetime: Utc::now(),
        last_price: 150.0,
        volume: 1000.0,
        bid_price: 149.5,
        bid_volume: 500.0,
        ask_price: 150.5,
        ask_volume: 600.0,
    };

    assert_eq!(tick.symbol, "AAPL");
    assert!(tick.last_price > 0.0);
    assert!(tick.volume > 0.0);
    assert!(tick.bid_price < tick.ask_price);
    assert!(tick.bid_volume > 0.0);
    assert!(tick.ask_volume > 0.0);
}

