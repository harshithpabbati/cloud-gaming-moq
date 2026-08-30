use moq_rs::protocol::message::{Message, MessageType};
use moq_rs::relay::client::ClientManager;
use tokio::sync::mpsc;
use uuid::Uuid;

#[test]
fn test_register_client() {
    let mut manager = ClientManager::new();
    let (sender, _receiver) = mpsc::channel(1);

    let client_id = manager.register(sender);

    assert!(manager.contains(client_id));
    assert_eq!(manager.len(), 1);
}

#[test]
fn test_register_multiple_clients() {
    let mut manager = ClientManager::new();
    let (sender_1, _receiver_1) = mpsc::channel(1);
    let (sender_2, _receiver_2) = mpsc::channel(1);
    let (sender_3, _receiver_3) = mpsc::channel(1);

    let client_1 = manager.register(sender_1);
    let client_2 = manager.register(sender_2);
    let client_3 = manager.register(sender_3);

    assert!(manager.contains(client_1));
    assert!(manager.contains(client_2));
    assert!(manager.contains(client_3));
    assert_eq!(manager.len(), 3);
}

#[test]
fn test_register_returns_unique_client_ids() {
    let mut manager = ClientManager::new();
    let (sender_1, _receiver_1) = mpsc::channel(1);
    let (sender_2, _receiver_2) = mpsc::channel(1);

    let client_1 = manager.register(sender_1);
    let client_2 = manager.register(sender_2);

    assert_ne!(client_1, client_2);
    assert_eq!(manager.len(), 2);
}

#[test]
fn test_remove_client() {
    let mut manager = ClientManager::new();
    let (sender_1, _receiver_1) = mpsc::channel(1);
    let (sender_2, _receiver_2) = mpsc::channel(1);

    let client_1 = manager.register(sender_1);
    let client_2 = manager.register(sender_2);

    manager.remove(client_1);

    assert!(!manager.contains(client_1));
    assert!(manager.contains(client_2));
    assert_eq!(manager.len(), 1);
}

#[test]
fn test_remove_unknown_client() {
    let mut manager = ClientManager::new();
    let (sender, _receiver) = mpsc::channel(1);

    let client_id = manager.register(sender);

    manager.remove(Uuid::new_v4());

    assert_eq!(manager.len(), 1);
    assert!(manager.contains(client_id));
}

#[test]
fn test_empty_manager() {
    let manager = ClientManager::new();

    assert!(manager.is_empty());
    assert_eq!(manager.len(), 0);
}

#[tokio::test]
async fn test_send_message_to_registered_client() {
    let mut manager = ClientManager::new();
    let (sender, mut receiver) = mpsc::channel(1);
    let client_id = manager.register(sender);
    let message = Message {
        message_type: MessageType::Data,
        channel_name: "game-123".to_string(),
        payload: b"Frame 1".to_vec(),
    };

    manager
        .send(client_id, message.clone())
        .await
        .expect("send should succeed");

    assert_eq!(receiver.recv().await, Some(message));
}

#[tokio::test]
async fn test_send_message_to_unknown_client() {
    let manager = ClientManager::new();
    let message = Message {
        message_type: MessageType::Data,
        channel_name: "game-123".to_string(),
        payload: b"Frame 1".to_vec(),
    };

    let result = manager.send(Uuid::new_v4(), message).await;

    assert_eq!(result, Err("client not found"));
}
