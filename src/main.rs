mod conn;
mod conn_manager;
mod response;

use conn::Connection;
use conn_manager::ConnectionManager;
use dotenvy::dotenv;
use reqwest::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    dotenv().ok();

    const COUNT: i32 = 10;

    let conn_map = Arc::new(ConnectionManager::new());
    let fetch_tasks = fetch_data(COUNT);

    let mut recv_tasks = Vec::new();

    // Call a concurrent async tasks
    for fetch in fetch_tasks {
        let response = fetch.await.unwrap().unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<response::Response>();
        let connection = Arc::new(Connection::new(response.id, tx));
        conn_map.clone().register(&connection);

        // sending message per connection.
        connection.send(response);

        let recv_task = {
            let conn = connection.clone();
            tokio::spawn(async move {
                while let Some(resp) = rx.recv().await {
                    println!("Receiver ID: {}, Response data: {:?}", conn.id, resp);
                }
            })
        };

        recv_tasks.push(recv_task);
    }

    for task in recv_tasks {
        let _ = task.await;
    }

    conn_map.shutdown_all().await;
}

fn fetch_data(count: i32) -> Vec<JoinHandle<Result<response::Response, Error>>> {
    let mut tasks = Vec::new();

    for id in 1..=count {
        let request_url = format!("https://jsonplaceholder.typicode.com/todos/{id}");
        tasks.push(tokio::spawn(async move {
            reqwest::get(request_url)
                .await
                .unwrap()
                .json::<response::Response>()
                .await
        }));
    }

    tasks
}
