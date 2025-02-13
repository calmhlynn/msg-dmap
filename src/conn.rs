use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub(super) struct Connection<T> {
    pub(super) id: u32,
    pub(super) uuid: Uuid,
    tx: UnboundedSender<T>,
}

impl<T> Connection<T> {
    pub(super) fn new(id: u32, tx: UnboundedSender<T>) -> Self {
        Self {
            id,
            uuid: Uuid::new_v4(),
            tx,
        }
    }

    #[inline]
    pub(super) fn send(&self, message: T) {
        let _ = self.tx.send(message);
    }

    pub(super) async fn shutdown(&self) {
        self.tx.closed().await;
    }
}
