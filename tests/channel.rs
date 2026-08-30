use moq_rs::relay::channel::ChannelManager;
use uuid::Uuid;

#[test]
fn test_subscribe_client() {
    let mut manager = ChannelManager::new();
    let client_id = Uuid::new_v4();

    manager.subscribe("game-123", client_id);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should exist");

    assert!(subscribers.contains(&client_id));
}

#[test]
fn test_multiple_clients_subscribe() {
    let mut manager = ChannelManager::new();
    let client_1 = Uuid::new_v4();
    let client_2 = Uuid::new_v4();
    let client_3 = Uuid::new_v4();

    manager.subscribe("game-123", client_1);
    manager.subscribe("game-123", client_2);
    manager.subscribe("game-123", client_3);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should exist");

    assert_eq!(subscribers.len(), 3);
    assert!(subscribers.contains(&client_1));
    assert!(subscribers.contains(&client_2));
    assert!(subscribers.contains(&client_3));
}

#[test]
fn test_same_client_subscribes_twice() {
    let mut manager = ChannelManager::new();
    let client_id = Uuid::new_v4();

    manager.subscribe("game-123", client_id);
    manager.subscribe("game-123", client_id);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should exist");

    assert_eq!(subscribers.len(), 1);
}

#[test]
fn test_different_channels() {
    let mut manager = ChannelManager::new();
    let client_1 = Uuid::new_v4();
    let client_2 = Uuid::new_v4();

    manager.subscribe("game-123", client_1);
    manager.subscribe("game-456", client_2);

    let game_123 = manager
        .subscribers("game-123")
        .expect("game-123 should exist");

    let game_456 = manager
        .subscribers("game-456")
        .expect("game-456 should exist");

    assert_eq!(game_123.len(), 1);
    assert!(game_123.contains(&client_1));

    assert_eq!(game_456.len(), 1);
    assert!(game_456.contains(&client_2));
}

#[test]
fn test_unsubscribe_client() {
    let mut manager = ChannelManager::new();
    let client_1 = Uuid::new_v4();
    let client_2 = Uuid::new_v4();

    manager.subscribe("game-123", client_1);
    manager.subscribe("game-123", client_2);

    manager.unsubscribe("game-123", client_1);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should still exist");

    assert_eq!(subscribers.len(), 1);
    assert!(!subscribers.contains(&client_1));
    assert!(subscribers.contains(&client_2));
}

#[test]
fn test_channel_removed_when_last_client_unsubscribes() {
    let mut manager = ChannelManager::new();
    let client_id = Uuid::new_v4();

    manager.subscribe("game-123", client_id);

    manager.unsubscribe("game-123", client_id);

    assert!(manager.subscribers("game-123").is_none());
}

#[test]
fn test_unsubscribe_unknown_client() {
    let mut manager = ChannelManager::new();
    let client_id = Uuid::new_v4();

    manager.subscribe("game-123", client_id);

    manager.unsubscribe("game-123", Uuid::new_v4());

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should still exist");

    assert_eq!(subscribers.len(), 1);
    assert!(subscribers.contains(&client_id));
}

#[test]
fn test_unsubscribe_unknown_channel() {
    let mut manager = ChannelManager::new();

    manager.unsubscribe("does-not-exist", Uuid::new_v4());

    assert!(manager.subscribers("does-not-exist").is_none());
}

#[test]
fn test_publish_client() {
    let mut manager = ChannelManager::new();
    let client_id = Uuid::new_v4();

    manager
        .publish("game-123", client_id)
        .expect("publish should succeed");

    let publisher = manager.publisher("game-123");

    assert_eq!(publisher, Some(client_id));
}

#[test]
fn test_multiple_publishers_not_allowed() {
    let mut manager = ChannelManager::new();
    let client_1 = Uuid::new_v4();
    let client_2 = Uuid::new_v4();

    manager
        .publish("game-123", client_1)
        .expect("first publish should succeed");

    let result = manager.publish("game-123", client_2);

    assert!(result.is_err());

    assert_eq!(manager.publisher("game-123"), Some(client_1));
}

#[test]
fn test_disconnect_releases_publisher_and_subscriptions() {
    let mut manager = ChannelManager::new();
    let publisher_id = Uuid::new_v4();
    let subscriber_id = Uuid::new_v4();

    manager.publish("game-123", publisher_id).unwrap();
    manager.subscribe("game-123", publisher_id);
    manager.subscribe("game-123", subscriber_id);

    manager.disconnect(publisher_id);

    assert_eq!(manager.publisher("game-123"), None);
    let subscribers = manager.subscribers("game-123").unwrap();
    assert!(!subscribers.contains(&publisher_id));
    assert!(subscribers.contains(&subscriber_id));
    assert!(manager.publish("game-123", Uuid::new_v4()).is_ok());
}
