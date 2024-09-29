#[macro_use] extern crate rocket;

mod routers;
mod matching_engine;
use routers::orders::{crypto_order_route, stock_order_route};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![crypto_order_route, stock_order_route])
}
