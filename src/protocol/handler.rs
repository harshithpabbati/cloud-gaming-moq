use crate::protocol::message::{Message, MessageType};
use crate::relay::channel::{ChannelManager, ClientId};

pub async fn handle_message(
    message: &Message,
    client_id: ClientId,
    channel_manager: &mut ChannelManager,
) {
    match message.message_type {
        MessageType::Subscribe => {
            handle_subscribe(message, client_id, channel_manager).await;
        }
        MessageType::Unsubscribe => {
            handle_unsubscribe(message, client_id, channel_manager).await;
        }
        MessageType::Publish => {
            handle_publish(message, client_id, channel_manager).await;
        }
        MessageType::Data => {
            handle_data(message).await;
        }
    }
}

async fn handle_subscribe(
    message: &Message,
    client_id: ClientId,
    channel_manager: &mut ChannelManager,
) {
    channel_manager.subscribe(&message.channel_name, client_id);

    println!(
        "Client {client_id} subscribed to '{}'",
        message.channel_name
    );
}

async fn handle_unsubscribe(
    message: &Message,
    client_id: ClientId,
    channel_manager: &mut ChannelManager,
) {
    channel_manager.unsubscribe(&message.channel_name, client_id);
    println!(
        "Client {client_id} unsubscribed from '{}'",
        message.channel_name
    );
}

async fn handle_publish(
    message: &Message,
    client_id: ClientId,
    channel_manager: &mut ChannelManager,
) {
    match channel_manager.publish(&message.channel_name, client_id) {
        Ok(()) => {
            println!(
                "Client {client_id} is publishing to '{}'",
                message.channel_name
            );
        }

        Err(error) => {
            println!(
                "Client {client_id} failed to publish to '{}': {error}",
                message.channel_name
            );
        }
    }
}

async fn handle_data(message: &Message) {
    println!(
        "DATA: channel='{}', payload={} bytes",
        message.channel_name,
        message.payload.len()
    );
}
