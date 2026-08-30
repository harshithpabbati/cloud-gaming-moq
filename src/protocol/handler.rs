use crate::protocol::message::{Message, MessageType};
use crate::relay::channel::{ChannelManager, ClientId};
use crate::relay::client::ClientManager;
use tokio::sync::Mutex;

pub async fn handle_message(
    message: &Message,
    client_id: ClientId,
    channel_manager: &Mutex<ChannelManager>,
    client_manager: &Mutex<ClientManager>,
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
            handle_data(message, client_id, channel_manager, client_manager).await;
        }
    }
}

async fn handle_subscribe(
    message: &Message,
    client_id: ClientId,
    channel_manager: &Mutex<ChannelManager>,
) {
    channel_manager
        .lock()
        .await
        .subscribe(&message.channel_name, client_id);

    println!(
        "Client {client_id} subscribed to '{}'",
        message.channel_name
    );
}

async fn handle_unsubscribe(
    message: &Message,
    client_id: ClientId,
    channel_manager: &Mutex<ChannelManager>,
) {
    channel_manager
        .lock()
        .await
        .unsubscribe(&message.channel_name, client_id);

    println!(
        "Client {client_id} unsubscribed from '{}'",
        message.channel_name
    );
}

async fn handle_publish(
    message: &Message,
    client_id: ClientId,
    channel_manager: &Mutex<ChannelManager>,
) {
    match channel_manager
        .lock()
        .await
        .publish(&message.channel_name, client_id)
    {
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

async fn handle_data(
    message: &Message,
    client_id: ClientId,
    channel_manager: &Mutex<ChannelManager>,
    client_manager: &Mutex<ClientManager>,
) {
    let subscriber_ids = {
        let channel_manager = channel_manager.lock().await;

        if channel_manager.publisher(&message.channel_name) != Some(client_id) {
            println!(
                "Client {client_id} cannot send DATA on '{}': not the publisher",
                message.channel_name
            );

            return;
        }

        println!(
            "DATA: channel='{}', payload={} bytes",
            message.channel_name,
            message.payload.len()
        );

        channel_manager
            .subscribers(&message.channel_name)
            .map(|subscribers| subscribers.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    };

    let senders = {
        let client_manager = client_manager.lock().await;

        subscriber_ids
            .into_iter()
            .filter_map(|subscriber_id| {
                client_manager
                    .sender(subscriber_id)
                    .map(|sender| (subscriber_id, sender))
            })
            .collect::<Vec<_>>()
    };

    for (subscriber_id, sender) in senders {
        if let Err(_error) = sender.send(message.clone()).await {
            println!(
                "Client {subscriber_id} could not receive data on '{}': connection is closed",
                message.channel_name
            );
        }
    }
}
