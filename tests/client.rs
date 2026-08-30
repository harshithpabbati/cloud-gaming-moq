use moq_rs::relay::client::ClientManager;
use uuid::Uuid;

#[test]
fn test_register_client() {
    let mut manager = ClientManager::new();

    let client_id = manager.register();

    assert!(manager.contains(client_id));
    assert_eq!(manager.len(), 1);
}

#[test]
fn test_register_multiple_clients() {
    let mut manager = ClientManager::new();

    let client_1 = manager.register();
    let client_2 = manager.register();
    let client_3 = manager.register();

    assert!(manager.contains(client_1));
    assert!(manager.contains(client_2));
    assert!(manager.contains(client_3));
    assert_eq!(manager.len(), 3);
}

#[test]
fn test_register_returns_unique_client_ids() {
    let mut manager = ClientManager::new();

    let client_1 = manager.register();
    let client_2 = manager.register();

    assert_ne!(client_1, client_2);
    assert_eq!(manager.len(), 2);
}

#[test]
fn test_remove_client() {
    let mut manager = ClientManager::new();

    let client_1 = manager.register();
    let client_2 = manager.register();

    manager.remove(client_1);

    assert!(!manager.contains(client_1));
    assert!(manager.contains(client_2));
    assert_eq!(manager.len(), 1);
}

#[test]
fn test_remove_unknown_client() {
    let mut manager = ClientManager::new();

    let client_id = manager.register();

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
