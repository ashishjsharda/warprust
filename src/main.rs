use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use warp::Filter;

struct Item {
    name: String,
    price: u32,
}

struct Store {
    grocery_list: Arc<RwLock<Items>>,
}

impl Store{
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;

}
