
//use crate::model::{Tick, OrderRequest, Trade, CancelRequest};
use crate::strategy::Tick;
use chrono::{DateTime, Utc, TimeZone};
use tokio;
pub mod order;
pub use order::{OrderRequest, CancelRequest};

pub trait Gateway {
    fn name(&self) -> &str;

    fn connect(&mut self) -> anyhow::Result<()>;

    fn subscribe(&mut self, symbols: Vec<String>) -> anyhow::Result<()>;

    fn send_order(&self, req: OrderRequest) -> anyhow::Result<String>;

    fn cancel_order(&mut self, req: CancelRequest) -> anyhow::Result<()>;

    fn query_account(&self) -> anyhow::Result<()>;

    fn query_position(&self) -> anyhow::Result<()>;

    fn read(&self)-> Tick;
}


pub struct MockGateway {
    pub name: String,
    pub subscribed_names : Vec<String>,
} 

impl Gateway for MockGateway {
    fn name(&self) -> &str {
        &self.name
    }

    fn connect(&mut self) -> anyhow::Result<()> {
        println!("Connecting to mock gateway...");
        Ok(())
    }

    fn subscribe(&mut self, symbols: Vec<String>) -> anyhow::Result<()> {
        for symbol in &symbols {

            println!("Subscribing to symbol: {}", symbol);
            self.subscribed_names.push(symbol.clone());
        }
        Ok(())
    }

    fn read(&self)-> Tick {
        std::thread::sleep(std::time::Duration::from_secs(1));
        Tick{
            symbol: "AAPL".to_string(),                
            datetime: Utc::now(),       
            last_price: 12.0f64,               
            volume: 1000.0f64,                   
            bid_price: 10.0f64,                
            bid_volume: 2000.0f64,               
            ask_price: 13.05f64,                
            ask_volume: 2300.0f64, 
        }

    } 

    fn send_order(&self, req: OrderRequest) -> anyhow::Result<String> {
        Ok("Ok".to_string())
    }

    fn cancel_order(&mut self, req: CancelRequest) -> anyhow::Result<()> {
        Ok(())
    }

    fn query_account(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn query_position(&self) -> anyhow::Result<()> {
        Ok(())
    }

}



#[cfg(test)]
mod tests {
    
    use super::*;
    use crate::engine::Engine;
    use crate::strategy::Strategy;
    //use crate::model::{Tick, Trade};
    use std::collections::HashMap;

    #[test]
    fn test_mock_gateway() {
        let mut gateway = MockGateway {
            name: "MockGateway".to_string(),
            subscribed_names: vec![],
        };

        gateway.connect().unwrap();
        gateway.subscribe(vec!["AAPL".to_string(), "GOOGL".to_string()]).unwrap();

        assert_eq!(gateway.subscribed_names, vec!["AAPL".to_string(), "GOOGL".to_string()]);

        for i in 0..5 {
            let tick = gateway.read();
            println!("Received tick: {:?}", tick);
        }

    }
}