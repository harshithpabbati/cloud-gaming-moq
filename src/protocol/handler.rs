use crate::protocol::message::{Message, MessageType};

pub async fn handle_message(message: Message) {
    match message.message_type {
        MessageType::Subscribe => {
            handle_subscribe(message).await;
        }
        MessageType::Unsubscribe => {
            handle_unsubscribe(message).await;
        }
        MessageType::Publish => {
            handle_publish(message).await;
        }
        MessageType::Data => {
            handle_data(message).await;
        }
    }
}

async fn handle_subscribe(message: Message) {
    println!("SUBSCRIBE: channel='{}'", message.channel_name);
}

async fn handle_unsubscribe(message: Message) {
    println!("UNSUBSCRIBE: channel='{}'", message.channel_name);
}

async fn handle_publish(message: Message) {
    println!(
        "PUBLISH: channel='{}', payload={} bytes",
        message.channel_name,
        message.payload.len()
    );
}

async fn handle_data(message: Message) {
    println!(
        "DATA: channel='{}', payload={} bytes",
        message.channel_name,
        message.payload.len()
    );
}
