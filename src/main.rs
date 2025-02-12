mod front;
mod kafka_pc;

use dashmap::DashMap;
use dotenvy::dotenv;
use front::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

type ResponseConnection = Arc<Connection<Response>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let conn_map: Arc<DashMap<Uuid, ResponseConnection>> = Arc::new(DashMap::new());
    let mut fetch_tasks = Vec::new();

    for id in 1..=10 {
        let request_url = format!("https://jsonplaceholder.typicode.com/todos/{id}");
        fetch_tasks.push(tokio::spawn(async move {
            reqwest::get(request_url)
                .await
                .unwrap()
                .json::<Response>()
                .await
        }));
    }

    let mut conn_tasks = Vec::new();

    for fetch in fetch_tasks {
        let response = fetch.await.unwrap().unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<Response>();
        let connection = Arc::new(Connection::new(tx));
        connection.register(&conn_map);
        // connection_map.insert(idx, connection.clone());

        let send_task = {
            let conn = connection.clone();
            let resp = response.clone();
            tokio::spawn(async move {
                conn.send(resp);
            })
        };

        let recv_task = {
            let conn = connection.clone();
            tokio::spawn(async move {
                while let Some(resp) = rx.recv().await {
                    println!("Channel {} received: {:?}", conn.id, resp);
                }
            })
        };

        conn_tasks.push(send_task);
        conn_tasks.push(recv_task);
    }

    for task in conn_tasks {
        let _ = task.await;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Response {
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}
