use std::collections::HashMap;
use rust_decimal::prelude::*;

use super::orderbook::{Orderbook, Order};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct StockTradingSymbol {
    symbol: String,
}

impl StockTradingSymbol {
    pub fn new(symbol: String) -> StockTradingSymbol {
        StockTradingSymbol { symbol }
    }
}

pub struct StockMatchingEngine {
    orderbooks: HashMap<StockTradingSymbol, Orderbook>,
}

impl StockMatchingEngine {
    pub fn new() -> StockMatchingEngine {
        StockMatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, symbol: StockTradingSymbol) {
        self.orderbooks.insert(symbol.clone(), Orderbook::new());
        println!("Opening new orderbook for market {:?}", symbol.symbol);
    }

    pub fn place_limit_order(&mut self, symbol: StockTradingSymbol, price: Decimal, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&symbol) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);

                println!("Placed limit order for market {:?} at price level {}", symbol.symbol, price);

                Ok(())
            }
            None => {
                Err(format!("Market {:?} does not exist", symbol.symbol))
            }
        }

    }
}
