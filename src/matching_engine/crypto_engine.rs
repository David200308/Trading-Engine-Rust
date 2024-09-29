use std::collections::HashMap;
use rust_decimal::prelude::*;

use super::orderbook::{Orderbook, Order};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct CryptoTradingPair {
    base: String,
    quote: String,
}

impl CryptoTradingPair {
    pub fn new(base: String, quote: String) -> CryptoTradingPair {
        CryptoTradingPair { base, quote }
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct CryptoMatchingEngine {
    orderbooks: HashMap<CryptoTradingPair, Orderbook>,
}

impl CryptoMatchingEngine {
    pub fn new() -> CryptoMatchingEngine {
        CryptoMatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: CryptoTradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: CryptoTradingPair, price: Decimal, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);

                println!("Placed limit order for market {:?} at price level {}", pair.to_string(), price);

                Ok(())
            }
            None => {
                Err(format!("Market {:?} does not exist", pair.to_string()))
            }
        }

    }
}
