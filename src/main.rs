use std::sync::{Arc, RwLock};
use warp::Filter;

struct Item {
    name: String,
    price: u32,
}

struct Store {
    grocery: Arc<RwLock<Items>>,
}

async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;

}
