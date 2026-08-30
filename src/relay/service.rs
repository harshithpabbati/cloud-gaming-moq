use std::fmt;

use tokio::sync::mpsc;

use crate::protocol::message::{Message, MessageType};

use super::channel::{ChannelManager, ClientId};
use super::client::ClientManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayError {
    UnknownClient,
    PublisherAlreadyExists,
    NotPublisher,
}

impl fmt::Display for RelayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownClient => formatter.write_str("client is not connected"),
            Self::PublisherAlreadyExists => formatter.write_str("channel already has a publisher"),
            Self::NotPublisher => formatter.write_str("client is not the channel publisher"),
        }
    }
}

impl std::error::Error for RelayError {}

#[derive(Default)]
pub struct Relay {
    clients: ClientManager,
    channels: ChannelManager,
}

impl Relay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, outbound: mpsc::Sender<Message>) -> ClientId {
        self.clients.register(outbound)
    }

    pub fn disconnect(&mut self, client_id: ClientId) {
        self.channels.disconnect(client_id);
        self.clients.remove(client_id);
    }

    pub fn handle_message(
        &mut self,
        client_id: ClientId,
        message: &Message,
    ) -> Result<Vec<mpsc::Sender<Message>>, RelayError> {
        if !self.clients.contains(client_id) {
            return Err(RelayError::UnknownClient);
        }

        match message.message_type {
            MessageType::Subscribe => {
                self.channels.subscribe(&message.channel_name, client_id);
                Ok(Vec::new())
            }
            MessageType::Unsubscribe => {
                self.channels.unsubscribe(&message.channel_name, client_id);
                Ok(Vec::new())
            }
            MessageType::Publish => self
                .channels
                .publish(&message.channel_name, client_id)
                .map(|()| Vec::new())
                .map_err(|_| RelayError::PublisherAlreadyExists),
            MessageType::Data => self.data_recipients(client_id, &message.channel_name),
        }
    }

    fn data_recipients(
        &self,
        client_id: ClientId,
        channel_name: &str,
    ) -> Result<Vec<mpsc::Sender<Message>>, RelayError> {
        if self.channels.publisher(channel_name) != Some(client_id) {
            return Err(RelayError::NotPublisher);
        }

        Ok(self
            .channels
            .subscribers(channel_name)
            .into_iter()
            .flatten()
            .filter_map(|subscriber_id| self.clients.sender(*subscriber_id))
            .collect())
    }
}
