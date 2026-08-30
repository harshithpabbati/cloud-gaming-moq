use moq_rs::relay::channel::ChannelManager;

#[test]
fn test_subscribe_client() {
    let mut manager = ChannelManager::new();

    manager.subscribe("game-123", 1);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should exist");

    assert!(subscribers.contains(&1));
}

#[test]
fn test_multiple_clients_subscribe() {
    let mut manager = ChannelManager::new();

    manager.subscribe("game-123", 1);
    manager.subscribe("game-123", 2);
    manager.subscribe("game-123", 3);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should exist");

    assert_eq!(subscribers.len(), 3);
    assert!(subscribers.contains(&1));
    assert!(subscribers.contains(&2));
    assert!(subscribers.contains(&3));
}

#[test]
fn test_same_client_subscribes_twice() {
    let mut manager = ChannelManager::new();

    manager.subscribe("game-123", 1);
    manager.subscribe("game-123", 1);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should exist");

    assert_eq!(subscribers.len(), 1);
}

#[test]
fn test_different_channels() {
    let mut manager = ChannelManager::new();

    manager.subscribe("game-123", 1);
    manager.subscribe("game-456", 2);

    let game_123 = manager
        .subscribers("game-123")
        .expect("game-123 should exist");

    let game_456 = manager
        .subscribers("game-456")
        .expect("game-456 should exist");

    assert_eq!(game_123.len(), 1);
    assert!(game_123.contains(&1));

    assert_eq!(game_456.len(), 1);
    assert!(game_456.contains(&2));
}

#[test]
fn test_unsubscribe_client() {
    let mut manager = ChannelManager::new();

    manager.subscribe("game-123", 1);
    manager.subscribe("game-123", 2);

    manager.unsubscribe("game-123", 1);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should still exist");

    assert_eq!(subscribers.len(), 1);
    assert!(!subscribers.contains(&1));
    assert!(subscribers.contains(&2));
}

#[test]
fn test_channel_removed_when_last_client_unsubscribes() {
    let mut manager = ChannelManager::new();

    manager.subscribe("game-123", 1);

    manager.unsubscribe("game-123", 1);

    assert!(manager.subscribers("game-123").is_none());
}

#[test]
fn test_unsubscribe_unknown_client() {
    let mut manager = ChannelManager::new();

    manager.subscribe("game-123", 1);

    manager.unsubscribe("game-123", 999);

    let subscribers = manager
        .subscribers("game-123")
        .expect("channel should still exist");

    assert_eq!(subscribers.len(), 1);
    assert!(subscribers.contains(&1));
}

#[test]
fn test_unsubscribe_unknown_channel() {
    let mut manager = ChannelManager::new();

    manager.unsubscribe("does-not-exist", 1);

    assert!(manager.subscribers("does-not-exist").is_none());
}

#[test]
fn test_publish_client() {
    let mut manager = ChannelManager::new();

    manager
        .publish("game-123", 1)
        .expect("publish should succeed");

    let publisher = manager.publisher("game-123");

    assert_eq!(publisher, Some(1));
}

#[test]
fn test_multiple_publishers_not_allowed() {
    let mut manager = ChannelManager::new();

    manager
        .publish("game-123", 1)
        .expect("first publish should succeed");

    let result = manager.publish("game-123", 2);

    assert!(result.is_err());

    assert_eq!(manager.publisher("game-123"), Some(1));
}
