use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub type ClientId = Uuid;

#[derive(Debug, Default)]
pub struct Channel {
    pub publisher: Option<ClientId>,
    pub subscribers: HashSet<ClientId>,
}

pub struct ChannelManager {
    channels: HashMap<String, Channel>,
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn publish(&mut self, channel_name: &str, client_id: ClientId) -> Result<(), &'static str> {
        let channel = self.channels.entry(channel_name.to_string()).or_default();
        if channel.publisher.is_some() {
            return Err("channel already has a publisher");
        }
        channel.publisher = Some(client_id);
        Ok(())
    }

    pub fn subscribe(&mut self, channel_name: &str, client_id: ClientId) {
        self.channels
            .entry(channel_name.to_string())
            .or_default()
            .subscribers
            .insert(client_id);
    }

    pub fn unsubscribe(&mut self, channel_name: &str, client_id: ClientId) {
        let should_remove = if let Some(channel) = self.channels.get_mut(channel_name) {
            channel.subscribers.remove(&client_id);
            channel.publisher.is_none() && channel.subscribers.is_empty()
        } else {
            false
        };

        if should_remove {
            self.channels.remove(channel_name);
        }
    }

    pub fn subscribers(&self, channel_name: &str) -> Option<&HashSet<ClientId>> {
        self.channels
            .get(channel_name)
            .map(|channel| &channel.subscribers)
    }

    pub fn publisher(&self, channel_name: &str) -> Option<ClientId> {
        self.channels
            .get(channel_name)
            .and_then(|channel| channel.publisher)
    }

    pub fn channel(&self, channel_name: &str) -> Option<&Channel> {
        self.channels.get(channel_name)
    }
}
