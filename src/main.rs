use crate::producer::produce;

mod producer;
mod consumer;

#[tokio::main]
async fn main() {
    // Kafka - Message Queue
    let producer = producer::create();
    produce(producer, String::from("Hello World, I am testing")).await;
    consumer::start().await;
}