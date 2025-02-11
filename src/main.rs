use std::sync::Arc;

use dashmap::DashMap;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;

type ConnMpsc = Arc<ConnectionMpsc<ResponseData>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connection_map: Arc<DashMap<Uuid, ConnMpsc>> = Arc::new(DashMap::new());
    let mut request_handles = Vec::new();

    for task_id in 1..=10 {
        let request_url = format!("https://jsonplaceholder.typicode.com/todos/{task_id}");
        request_handles.push(tokio::spawn(async move {
            reqwest::get(request_url)
                .await
                .unwrap()
                .json::<ResponseData>()
                .await
        }));
    }

    let mut task_handles = Vec::new();

    for request_handle in request_handles {
        let response_data = request_handle.await.unwrap().unwrap();

        let (sender, mut receiver) = mpsc::unbounded_channel::<ResponseData>();
        let idx = Uuid::new_v4();
        let connection_mpsc = Arc::new(ConnectionMpsc::new(idx, sender));
        connection_map.insert(idx, connection_mpsc.clone());

        let send_task_handle = {
            let connection_mpsc_clone = connection_mpsc.clone();
            let response_data_clone = response_data.clone();
            tokio::spawn(async move {
                connection_mpsc_clone.send(response_data_clone);
            })
        };

        let receive_task_handle = {
            let channel_id = connection_mpsc.id;
            tokio::spawn(async move {
                while let Some(received_data) = receiver.recv().await {
                    println!("Channel {} received: {:?}", channel_id, received_data);
                }
            })
        };

        task_handles.push(send_task_handle);
        task_handles.push(receive_task_handle);
    }

    println!("task length: {:?}", task_handles.len());

    for task_handle in task_handles {
        let _ = task_handle.await;
    }
}

struct ConnectionMpsc<T> {
    id: Uuid,
    sender: UnboundedSender<T>,
}

impl<T> ConnectionMpsc<T> {
    fn new(id: Uuid, sender: UnboundedSender<T>) -> Self {
        Self { id, sender }
    }

    fn send(&self, message: T) {
        let _ = self.sender.send(message);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ResponseData {
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}
