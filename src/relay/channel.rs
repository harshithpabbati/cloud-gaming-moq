use std::collections::{HashMap, HashSet};

pub type ClientId = u64;

pub struct ChannelManager {
    channels: HashMap<String, HashSet<ClientId>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, channel_name: &str, client_id: ClientId) {
        self.channels
            .entry(channel_name.to_string())
            .or_default()
            .insert(client_id);
    }

    pub fn unsubscribe(&mut self, channel_name: &str, client_id: ClientId) {
        if let Some(subscribers) = self.channels.get_mut(channel_name) {
            subscribers.remove(&client_id);

            if subscribers.is_empty() {
                self.channels.remove(channel_name);
            }
        }
    }

    pub fn subscribers(&self, channel_name: &str) -> Option<&HashSet<ClientId>> {
        self.channels.get(channel_name)
    }
}
