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

static BROKER: Lazy<String> =
    Lazy::new(|| env::var("KAFKA_BROKER").expect("env KAFKA_BROKER is not set"));

static TOPIC: Lazy<String> =
    Lazy::new(|| env::var("KAFKA_TOPIC").expect("env KAFKA_TOPIC is not set"));

static GROUP_ID: Lazy<String> =
    Lazy::new(|| env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "my-group".to_string()));

static USERNAME: Lazy<String> =
    Lazy::new(|| env::var("KAFKA_USERNAME").expect("env KAFKA_USERNAME is not set"));

static PASSWORD: Lazy<String> =
    Lazy::new(|| env::var("KAFKA_PASSWORD").expect("env KAFKA_PASSWORD is not set"));

pub struct KafkaConsumer {
    pub consumer: StreamConsumer,
    pub topic: String,
}

impl KafkaConsumer {
    fn new() -> Result<Self, KafkaError> {
        let broker = &*BROKER;
        let topic = &*TOPIC;
        let group_id = &*GROUP_ID;
        let username = &*USERNAME;
        let password = &*PASSWORD;

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .set("group.id", group_id)
            .set("allow.auto.create.topics", "true")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "SCRAM-SHA-256")
            .set("sasl.username", username)
            .set("sasl.password", password)
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
        println!(
            "Consuming messages from topic '{}' filtering for key '{}'",
            self.topic, filter_key
        );
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

        let username = &*USERNAME;
        let password = &*PASSWORD;

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .set("message.timeout.ms", "5000")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "SCRAM-SHA-256")
            .set("sasl.username", username)
            .set("sasl.password", password)
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

        println!("try to send record");

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(delivery) => Ok(format!("Message delivery: {:?}", delivery)),
            Err((e, _)) => Err(format!("Error delivering message: {:?}", e)),
        }
    }
}
