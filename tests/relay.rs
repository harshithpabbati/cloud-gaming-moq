use moq_rs::protocol::message::{Message, MessageType};
use moq_rs::relay::{Relay, RelayError};
use tokio::sync::mpsc;

fn message(message_type: MessageType, channel_name: &str, payload: &[u8]) -> Message {
    Message {
        message_type,
        channel_name: channel_name.to_string(),
        payload: payload.to_vec(),
    }
}

#[tokio::test]
async fn relay_forwards_data_to_subscribers() {
    let mut relay = Relay::new();
    let (publisher_tx, _) = mpsc::channel(1);
    let (subscriber_tx, mut subscriber_rx) = mpsc::channel(1);
    let publisher_id = relay.connect(publisher_tx);
    let subscriber_id = relay.connect(subscriber_tx);

    relay
        .handle_message(
            publisher_id,
            &message(MessageType::Publish, "game-123", &[]),
        )
        .unwrap();
    relay
        .handle_message(
            subscriber_id,
            &message(MessageType::Subscribe, "game-123", &[]),
        )
        .unwrap();

    let data = message(MessageType::Data, "game-123", b"frame-1");
    let recipients = relay.handle_message(publisher_id, &data).unwrap();

    assert_eq!(recipients.len(), 1);
    recipients[0].send(data.clone()).await.unwrap();
    assert_eq!(subscriber_rx.recv().await, Some(data));
}

#[tokio::test]
async fn relay_rejects_data_from_non_publisher() {
    let mut relay = Relay::new();
    let (publisher_tx, _) = mpsc::channel(1);
    let (subscriber_tx, _) = mpsc::channel(1);
    let publisher_id = relay.connect(publisher_tx);
    let subscriber_id = relay.connect(subscriber_tx);

    relay
        .handle_message(
            publisher_id,
            &message(MessageType::Publish, "game-123", &[]),
        )
        .unwrap();

    assert!(matches!(
        relay.handle_message(
            subscriber_id,
            &message(MessageType::Data, "game-123", b"data")
        ),
        Err(RelayError::NotPublisher)
    ));
}

#[tokio::test]
async fn disconnect_releases_publisher_and_removes_subscriber() {
    let mut relay = Relay::new();
    let (publisher_tx, _) = mpsc::channel(1);
    let (subscriber_tx, _) = mpsc::channel(1);
    let (replacement_tx, _) = mpsc::channel(1);
    let publisher_id = relay.connect(publisher_tx);
    let subscriber_id = relay.connect(subscriber_tx);
    let replacement_id = relay.connect(replacement_tx);

    relay
        .handle_message(
            publisher_id,
            &message(MessageType::Publish, "game-123", &[]),
        )
        .unwrap();
    relay
        .handle_message(
            subscriber_id,
            &message(MessageType::Subscribe, "game-123", &[]),
        )
        .unwrap();

    relay.disconnect(publisher_id);
    relay.disconnect(subscriber_id);

    relay
        .handle_message(
            replacement_id,
            &message(MessageType::Publish, "game-123", &[]),
        )
        .unwrap();
    assert!(
        relay
            .handle_message(
                replacement_id,
                &message(MessageType::Data, "game-123", b"data"),
            )
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn relay_rejects_messages_from_disconnected_clients() {
    let mut relay = Relay::new();
    let (sender, _) = mpsc::channel(1);
    let client_id = relay.connect(sender);
    relay.disconnect(client_id);

    assert!(matches!(
        relay.handle_message(client_id, &message(MessageType::Subscribe, "game-123", &[])),
        Err(RelayError::UnknownClient)
    ));
}
