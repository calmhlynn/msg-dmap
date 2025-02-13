use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Response {
    pub(super) id: u32,
    user_id: u32,
    title: String,
    completed: bool,
    // operator: u8,
}

impl Response {
    pub(super) async fn process(&self) {
        match self.id {
            0 => Self::zero(self).await,
            1 => Self::one(self).await,
            _ => eprintln!("Unknown operator: {}", self.id),
        }
    }

    async fn zero(&self) {
        todo!();
    }

    async fn one(&self) {
        todo!();
    }
}
