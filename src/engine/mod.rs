//mod crate::strategy;
use crate::strategy::{Strategy, Tick};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Direction {
    Buy,
    Sell,
}
#[derive(Debug, Clone)]
pub struct Trade {
    pub symbol: String,
    pub order_id: String,
    pub price: f64,
    pub volume: f64,
    pub direction: Direction, // Buy / Sell
    pub timestamp: DateTime<Utc>,
}

pub struct Engine {
    pub name: String,
    pub strategies: HashMap<String, Box<dyn Strategy>>,
}

impl Engine {
    pub fn new(name: &str) -> Self {
        Engine {
            name: name.to_string(),
            strategies: HashMap::new(),
        }
    }

    pub fn get_strategy_mut(&mut self, strategy_name: &str) -> Option<&mut Box<dyn Strategy>> {
        self.strategies.get_mut(strategy_name)
    }

    pub fn register_strategy(&mut self, strategy_name: String, strategy: Box<dyn Strategy>) {
        self.strategies.insert(strategy_name, strategy);
    }

    pub fn on_tick(&mut self, tick: &Tick) -> anyhow::Result<()> {
        for strategy in self.strategies.values_mut() {
            strategy.on_tick(tick)?;
        }
        Ok(())
    }
    pub fn on_trade(&self, trade: &Trade) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn send_order(&self, order: &Trade) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn run_backtest(&mut self, iterator: impl Iterator<Item = Tick>) -> Result<()> {
        for trade in iterator {
            self.on_tick(&trade)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct NaiveStrategy {
        ticks: Vec<Tick>,

    }

    impl Strategy for NaiveStrategy {
        fn on_tick(&mut self, tick: &Tick) -> anyhow::Result<()> {
            self.ticks.push(tick.clone());
            println!("{}", tick.symbol);
            Ok(())
        }
    }

    pub struct TestDatasource {
        data: Vec<Tick>,
    }
    
    impl IntoIterator for TestDatasource {
        type Item = Tick;
        type IntoIter = IntoIter; 
    
        fn into_iter(self) -> Self::IntoIter {
            IntoIter {
                data: self.data, 
            }
        }
    }

    pub struct IntoIter {
        data: Vec<Tick>,
    }
    
    impl Iterator for IntoIter {
        type Item = Tick;
    
        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }
    
    fn construct_datasource() ->TestDatasource {
        let datasource = TestDatasource {
            data: vec![
                Tick {
                    symbol: "AAPL".to_string(),
                    datetime: Utc::now(),
                    last_price: 150.0,
                    volume: 1000.0,
                    bid_price: 149.5,
                    bid_volume: 500.0,
                    ask_price: 150.5,
                    ask_volume: 600.0,
                },
                Tick {
                    symbol: "AAPL".to_string(),
                    datetime: Utc::now(),
                    last_price: 150.0,
                    volume: 1000.0,
                    bid_price: 149.5,
                    bid_volume: 500.0,
                    ask_price: 150.5,
                    ask_volume: 600.0,
                }
            ],
        };
        datasource
    }

    #[test]
    fn test_engine() {

        let mut test_datasource = construct_datasource();
        let mut engine = Engine::new("hello");
        let strategy = NaiveStrategy { ticks: vec![] };
        engine.register_strategy("naive".to_string(), Box::new(strategy));

        let iterator = test_datasource.into_iter();
        engine.run_backtest(iterator);

    }
}
