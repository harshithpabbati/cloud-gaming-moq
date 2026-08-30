use std::collections::HashSet;

use super::channel::ClientId;

pub struct ClientManager {
    clients: HashSet<ClientId>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: HashSet::new(),
        }
    }

    pub fn register(&mut self) -> ClientId {
        let client_id = ClientId::new_v4();
        self.clients.insert(client_id);
        client_id
    }

    pub fn remove(&mut self, client_id: ClientId) {
        self.clients.remove(&client_id);
    }

    pub fn contains(&self, client_id: ClientId) -> bool {
        self.clients.contains(&client_id)
    }

    pub fn len(&self) -> usize {
        self.clients.len()
    }

    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}
