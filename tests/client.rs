use moq_rs::relay::client::ClientManager;

#[test]
fn test_register_client() {
    let mut manager = ClientManager::new();

    manager.register(1);

    assert!(manager.contains(1));
    assert_eq!(manager.len(), 1);
}

#[test]
fn test_register_multiple_clients() {
    let mut manager = ClientManager::new();

    manager.register(1);
    manager.register(2);
    manager.register(3);

    assert!(manager.contains(1));
    assert!(manager.contains(2));
    assert!(manager.contains(3));
    assert_eq!(manager.len(), 3);
}

#[test]
fn test_register_same_client_twice() {
    let mut manager = ClientManager::new();

    manager.register(1);
    manager.register(1);

    assert_eq!(manager.len(), 1);
}

#[test]
fn test_remove_client() {
    let mut manager = ClientManager::new();

    manager.register(1);
    manager.register(2);

    manager.remove(1);

    assert!(!manager.contains(1));
    assert!(manager.contains(2));
    assert_eq!(manager.len(), 1);
}

#[test]
fn test_remove_unknown_client() {
    let mut manager = ClientManager::new();

    manager.register(1);

    manager.remove(999);

    assert_eq!(manager.len(), 1);
    assert!(manager.contains(1));
}

#[test]
fn test_empty_manager() {
    let manager = ClientManager::new();

    assert!(manager.is_empty());
    assert_eq!(manager.len(), 0);
}
