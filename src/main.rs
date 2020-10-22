extern crate mongodb;
#[macro_use]
extern crate bson;

mod storage;
mod models;

use storage::transport::Storage;
use tokio::stream::StreamExt;

#[tokio::main]
pub async fn main() {
    println!("Hello, spectrum!");

    //"mongodb://spectrum:ubq4lyfe@mlocalhost:27017"

    let backend = Storage::new().await;

    backend.set_key().await;

    let mut c = backend.get_key().await.unwrap();

    while let Some(x) = c.next().await {
        println!("{:?}", x)
    }
}