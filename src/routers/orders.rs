use rocket::serde::{json::Json, Deserialize, Serialize};
use crate::matching_engine::orderbook::{BidOrAsk, Order};
use crate::matching_engine::crypto_engine::{CryptoMatchingEngine, CryptoTradingPair};
use crate::matching_engine::stock_engine::{StockMatchingEngine, StockTradingSymbol};
use rust_decimal::Decimal;
// use rust_decimal_macros::dec;

#[derive(Deserialize)]
pub struct CryptoOrderRequest {
    pub order_type: String,
    pub price: f64,
    pub amount: Decimal,
    pub trading_pair_base: String,
    pub trading_pair_quote: String,
}

#[derive(Deserialize)]
pub struct StockOrderRequest {
    pub order_type: String,
    pub price: f64,
    pub amount: Decimal,
    pub trading_symbol: String,
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[post("/crypto_order", format = "json", data = "<crypto_order_request>")]
pub async fn crypto_order_route(crypto_order_request: Json<CryptoOrderRequest>) -> Json<Response> {
    let mut engine = CryptoMatchingEngine::new();
    let pair = CryptoTradingPair::new(
        crypto_order_request.trading_pair_base.to_uppercase().to_string(), 
        crypto_order_request.trading_pair_quote.to_uppercase().to_string()
    );

    engine.add_new_market(pair.clone());

    let order = Order::new(match crypto_order_request.order_type.as_str() {
        "buy" => BidOrAsk::Bid,
        "sell" => BidOrAsk::Ask,
        _ => return Json(Response { message: "Invalid order type".to_string() }),
    }, crypto_order_request.price);

    match engine.place_limit_order(pair, crypto_order_request.amount, order) {
        Ok(_) => Json(Response { message: "Crypto order placed successfully".to_string() }),
        Err(e) => Json(Response { message: format!("Error placing order: {}", e) }),
    }
}

#[post("/stock_order", format = "json", data = "<stock_order_request>")]
pub async fn stock_order_route(stock_order_request: Json<StockOrderRequest>) -> Json<Response> {
    let mut stock_engine = StockMatchingEngine::new();
    let symbol = StockTradingSymbol::new(stock_order_request.trading_symbol.to_uppercase().to_string());
    stock_engine.add_new_market(symbol.clone());

    let order = Order::new(match stock_order_request.order_type.as_str() {
        "buy" => BidOrAsk::Bid,
        "sell" => BidOrAsk::Ask,
        _ => return Json(Response { message: "Invalid order type".to_string() }),
    }, stock_order_request.price);

    match stock_engine.place_limit_order(symbol, stock_order_request.amount, order) {
        Ok(_) => Json(Response { message: "Stock order placed successfully".to_string() }),
        Err(e) => Json(Response { message: format!("Error placing order: {}", e) }),
    }
}
