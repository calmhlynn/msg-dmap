use std::sync::Arc;

use dashmap::DashMap;
use uuid::Uuid;

use super::Connection;

pub(crate) struct ConnectionManager<T>(DashMap<Uuid, Arc<Connection<T>>>);

impl<T> ConnectionManager<T> {
    pub(crate) fn new() -> Self {
        Self(DashMap::new())
    }

    pub(crate) fn register(&self, conn: &Arc<Connection<T>>) {
        self.0.insert(conn.uuid, conn.clone());
    }

    pub(crate) fn unregister(&self, id: &Uuid) {
        self.0.remove(id);
    }

    pub(crate) fn find_by_conn_id(&self, client_id: u32) -> Option<Arc<Connection<T>>> {
        self.0.iter().find_map(|entry| {
            if entry.value().id == client_id {
                Some(entry.value().clone())
            } else {
                None
            }
        })
    }

    pub(crate) async fn shutdown_all(&self) {
        for entry in self.0.iter() {
            entry.value().shutdown().await;
        }
    }
}
