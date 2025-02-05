mod pc;

use dotenvy::dotenv;
use pc::KafkaProducer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let producer = KafkaProducer::new().expect("Failed to create KafkaProducer");

    let key = String::from("test");
    let payload = String::from("payload");

    let msg = producer.send_record(key, payload).await;

    println!("msg: {:?}", msg);
}
