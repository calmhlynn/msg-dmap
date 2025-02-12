use std::{env, time::Duration};

use chrono::Utc;
use log::warn;
use once_cell::sync::Lazy;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};

macro_rules! env_lazy {
    ($name:ident, $var:expr) => {
        static $name: Lazy<String> =
            Lazy::new(|| env::var($var).expect(concat!("env ", $var, " is not set")));
    };
}

env_lazy!(BROKER, "KAFKA_BROKER");
env_lazy!(TOPIC, "KAFKA_TOPIC");
env_lazy!(GROUP_ID, "KAFKA_GROUP_ID");
env_lazy!(USERNAME, "KAFKA_USERNAME");
env_lazy!(PASSWORD, "KAFKA_PASSWORD");

pub struct KafkaConsumer {
    pub consumer: StreamConsumer,
    pub topic: String,
}

impl KafkaConsumer {
    fn new() -> Result<Self, KafkaError> {
        let broker = &*BROKER;
        let topic = &*TOPIC;
        let group_id = &*GROUP_ID;

        // let username = &*USERNAME;
        // let password = &*PASSWORD;

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .set("group.id", group_id)
            // .set("allow.auto.create.topics", "true")
            // .set("security.protocol", "SASL_PLAINTEXT")
            // .set("sasl.mechanism", "SCRAM-SHA-256")
            // .set("sasl.username", username)
            // .set("sasl.password", password)
            .set("enable.auto.commit", "true")
            .set("session.timeout.ms", "6000")
            .create()?;

        consumer
            .subscribe(&[&topic])
            .expect("Failed to subscribe to topic");

        Ok(Self {
            consumer,
            topic: topic.to_string(),
        })
    }

    async fn consume_messages(&self, filter_key: &str) {
        loop {
            match self.consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(m) => {
                    if let Some(key_bytes) = m.key() {
                        if key_bytes == filter_key.as_bytes() {
                            println!("payload: {:?}", m.payload());
                        }
                    };
                }
            }
        }
    }
}

pub struct KafkaProducer {
    pub producer: FutureProducer,
    pub topic: String,
}

impl KafkaProducer {
    pub fn new() -> Result<Self, KafkaError> {
        let broker = &*BROKER;
        let topic = &*TOPIC;

        // let username = &*USERNAME;
        // let password = &*PASSWORD;

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .set("message.timeout.ms", "5000")
            // .set("security.protocol", "SASL_PLAINTEXT")
            // .set("sasl.mechanism", "SCRAM-SHA-256")
            // .set("sasl.username", username)
            // .set("sasl.password", password)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }

    pub async fn send_record(self, key: String, payload: String) -> Result<String, String> {
        let record = FutureRecord::to(self.topic.as_str())
            .key(&key)
            .payload(&payload)
            .timestamp(Utc::now().timestamp());

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(delivery) => Ok(format!("Message delivery: {:?}", delivery)),
            Err((e, _)) => Err(format!("Error delivering message: {:?}", e)),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use tokio::time::timeout;
//
//     use super::*;
//
//     fn setup_test_env() {
//         env_lazy!(BROKER, "KAFKA_BROKER");
//         env_lazy!(TOPIC, "KAFKA_TOPIC");
//         env_lazy!(GROUP_ID, "KAFKA_GROUP_ID");
//         env_lazy!(USERNAME, "KAFKA_USERNAME");
//         env_lazy!(PASSWORD, "KAFKA_PASSWORD");
//     }
//
//     #[tokio::test]
//     async fn test_kafka_producer_send_record() {
//         setup_test_env();
//
//         let producer = KafkaProducer::new().expect("Failed to create KafkaProducer");
//
//         let key = String::from("test_key");
//         let payload = String::from("test_payload");
//
//         match producer.send_record(key, payload).await {
//             Ok(delivery) => {
//                 println!("Producer sent message successfully: {}", delivery);
//                 assert!(delivery.contains("Message delivery"));
//             }
//             Err(e) => {
//                 eprintln!("Producer failed to send message: {e}");
//                 panic!("Test failed: send_record returned an error");
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn test_kafka_consume_messages_timeout() {
//         setup_test_env();
//
//         let consumer = KafkaConsumer::new().expect("Failed to create KafkaConsumer");
//
//         let result = timeout(
//             Duration::from_secs(2),
//             consumer.consume_messages("test_key"),
//         )
//         .await;
//         assert!(
//             result.is_err(),
//             "consume_messages did not timeout as expected"
//         );
//     }
// }
