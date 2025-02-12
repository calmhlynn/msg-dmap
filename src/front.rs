use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub(crate) struct Connection<T> {
    pub(crate) id: Uuid,
    sender: UnboundedSender<T>,
}

impl<T> Connection<T> {
    pub(crate) fn new(sender: UnboundedSender<T>) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
        }
    }

    pub(crate) fn register(self: &Arc<Self>, connection_map: &Arc<DashMap<Uuid, Arc<Self>>>) {
        connection_map.insert(self.id, self.clone());
    }

    pub(crate) fn send(&self, message: T) {
        let _ = self.sender.send(message);
    }
}
