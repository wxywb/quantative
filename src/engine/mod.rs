//mod crate::strategy;
use crate::{gateway::order, strategy::{self, Strategy, Tick, Trade}};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;
use crate::gateway::{Gateway, MockGateway, OrderRequest};



pub struct Engine {
    pub name: String,
    pub strategies: HashMap<String, Box<dyn Strategy>>,
    pub gateway: Box<dyn Gateway>,
}

impl Engine {
    pub fn new(name: &str) -> Self {
        Engine {
            name: name.to_string(),
            strategies: HashMap::new(),
            gateway: Box::new(MockGateway {
                name: "mock_gateway".to_string(),
                subscribed_names: vec![],
            }),
        }
    }

    pub fn get_strategy_mut(&mut self, strategy_name: &str) -> Option<&mut Box<dyn Strategy>> {
        self.strategies.get_mut(strategy_name)
    }

    pub fn register_strategy(&mut self, strategy_name: String, strategy: Box<dyn Strategy>) {
        self.strategies.insert(strategy_name, strategy);
    }

    pub fn on_tick(&mut self, tick: &Tick) -> HashMap<String, Option<Trade>> {
        let mut trades_by_name = HashMap::new();

        for (strategy_name, strategy) in &mut self.strategies {
            let trade_signal = strategy.on_tick(tick);
            trades_by_name.insert(strategy_name.clone(), trade_signal);
        }
        trades_by_name
    }

    pub fn on_trade(&self, trade: &Trade) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn send_order(&self, order: &Trade) -> anyhow::Result<()> {

        
        let side = match order.direction {
            strategy::Direction::Buy => order::OrderSide::Buy,
            strategy::Direction::Sell => order::OrderSide::Sell,
        };
        let order_request  = OrderRequest {
            symbol: order.symbol.clone(),
            price: order.price,
            volume: order.volume,
            side: side,
            order_type:order::OrderType::Limit,
            gateway: Some(self.gateway.name().to_string()),
            time_in_force: Some(order::TimeInForce::GTC),
            stop_price: Some(0f64),
        };

        let result =self.gateway.send_order(order_request);
        match result {
            Ok(order_id) => {
                println!("Order sent successfully: {}", order_id);
                self.on_trade(order);
                return Ok(());
            }
            Err(e) => {
                println!("Failed to send order: {}", e);
                return Err(e);
            }
        }
    }

    pub fn run_backtest(&mut self, iterator: impl Iterator<Item = Tick>) -> Result<()> {
        for tick in iterator {
            let trades_by_name = self.on_tick(&tick);

            for(strategy_name, trade) in &trades_by_name {


                if let Some(trade) = trade {
                    self.send_order(&trade)?;
                }
            }
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

    use crate::strategy::Direction;

    impl Strategy for NaiveStrategy {
        fn on_tick(&mut self, tick: &Tick) -> Option<Trade> {
            let tick_clone = tick.clone();
            self.ticks.push(tick_clone);
            let trade = Trade {
                symbol: tick.symbol.clone(),
                order_id: "order_123".to_string(),
                price: tick.last_price,
                volume: tick.volume,
                direction: Direction::Buy,
                timestamp: tick.datetime,
            };
            Some(trade)
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
