use std::collections::HashMap;

use super::channel::ClientId;
use crate::protocol::message::Message;
use tokio::sync::mpsc;

struct ClientConnection {
    outbound: mpsc::Sender<Message>,
}

#[derive(Default)]
pub struct ClientManager {
    clients: HashMap<ClientId, ClientConnection>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, outbound: mpsc::Sender<Message>) -> ClientId {
        let client_id = ClientId::new_v4();
        self.clients
            .insert(client_id, ClientConnection { outbound });
        client_id
    }

    pub fn remove(&mut self, client_id: ClientId) {
        self.clients.remove(&client_id);
    }

    pub fn contains(&self, client_id: ClientId) -> bool {
        self.clients.contains_key(&client_id)
    }

    pub fn sender(&self, client_id: ClientId) -> Option<mpsc::Sender<Message>> {
        self.clients
            .get(&client_id)
            .map(|client| client.outbound.clone())
    }

    pub async fn send(&self, client_id: ClientId, message: Message) -> Result<(), &'static str> {
        let sender = self.sender(client_id).ok_or("client not found")?;

        sender
            .send(message)
            .await
            .map_err(|_| "client connection is closed")
    }

    pub fn len(&self) -> usize {
        self.clients.len()
    }

    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}
