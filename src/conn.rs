use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub(crate) struct Connection<T> {
    pub(crate) id: u32,
    pub(crate) uuid: Uuid,
    tx: UnboundedSender<T>,
}

impl<T> Connection<T> {
    pub(crate) fn new(id: u32, tx: UnboundedSender<T>) -> Self {
        Self {
            id,
            uuid: Uuid::new_v4(),
            tx,
        }
    }

    #[inline]
    pub(crate) fn send(&self, message: T) {
        let _ = self.tx.send(message);
    }

    pub(crate) async fn shutdown(&self) {
        self.tx.closed().await;
    }
}
