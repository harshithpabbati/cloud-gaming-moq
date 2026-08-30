use moq_rs::protocol::handler::handle_message;
use moq_rs::protocol::message::{Message, MessageType};
use moq_rs::relay::channel::ChannelManager;
use moq_rs::relay::client::ClientManager;
use tokio::sync::{Mutex, mpsc};

fn message(message_type: MessageType, channel_name: &str, payload: &[u8]) -> Message {
    Message {
        message_type,
        channel_name: channel_name.to_string(),
        payload: payload.to_vec(),
    }
}

#[tokio::test]
async fn test_publish() {
    let channels = Mutex::new(ChannelManager::new());
    let clients = Mutex::new(ClientManager::new());

    let (sender, _) = mpsc::channel(10);

    let publisher_id = {
        let mut clients = clients.lock().await;
        clients.register(sender)
    };

    let msg = message(MessageType::Publish, "game-123", &[]);

    handle_message(&msg, publisher_id, &channels, &clients).await;

    let channels = channels.lock().await;

    assert_eq!(channels.publisher("game-123"), Some(publisher_id));
}

#[tokio::test]
async fn test_subscribe() {
    let channels = Mutex::new(ChannelManager::new());
    let clients = Mutex::new(ClientManager::new());

    let (sender, _) = mpsc::channel(10);

    let client_id = {
        let mut clients = clients.lock().await;
        clients.register(sender)
    };

    let msg = message(MessageType::Subscribe, "game-123", &[]);

    handle_message(&msg, client_id, &channels, &clients).await;

    let channels = channels.lock().await;

    let subscribers = channels
        .subscribers("game-123")
        .expect("channel should exist");

    assert!(subscribers.contains(&client_id));
}

#[tokio::test]
async fn test_unsubscribe() {
    let channels = Mutex::new(ChannelManager::new());
    let clients = Mutex::new(ClientManager::new());

    let (sender, _) = mpsc::channel(10);

    let client_id = {
        let mut clients = clients.lock().await;
        clients.register(sender)
    };

    {
        let mut channels = channels.lock().await;
        channels.subscribe("game-123", client_id);
    }

    let msg = message(MessageType::Unsubscribe, "game-123", &[]);

    handle_message(&msg, client_id, &channels, &clients).await;

    let channels = channels.lock().await;

    assert!(channels.subscribers("game-123").is_none());
}

#[tokio::test]
async fn test_data_forwarded_to_subscriber() {
    let channels = Mutex::new(ChannelManager::new());
    let clients = Mutex::new(ClientManager::new());

    let (publisher_sender, _) = mpsc::channel(10);
    let (subscriber_sender, mut subscriber_receiver) = mpsc::channel(10);

    let (publisher_id, subscriber_id) = {
        let mut clients = clients.lock().await;

        let publisher_id = clients.register(publisher_sender);
        let subscriber_id = clients.register(subscriber_sender);

        (publisher_id, subscriber_id)
    };

    {
        let mut channels = channels.lock().await;

        channels
            .publish("game-123", publisher_id)
            .expect("publisher should be registered");

        channels.subscribe("game-123", subscriber_id);
    }

    let msg = message(MessageType::Data, "game-123", b"game-frame");

    handle_message(&msg, publisher_id, &channels, &clients).await;

    let received = subscriber_receiver
        .recv()
        .await
        .expect("subscriber should receive DATA");

    assert_eq!(received.channel_name, "game-123");
    assert_eq!(received.payload, b"game-frame");
}

#[tokio::test]
async fn test_data_forwarded_to_multiple_subscribers() {
    let channels = Mutex::new(ChannelManager::new());
    let clients = Mutex::new(ClientManager::new());

    let (publisher_sender, _) = mpsc::channel(10);
    let (sender_a, mut receiver_a) = mpsc::channel(10);
    let (sender_b, mut receiver_b) = mpsc::channel(10);

    let (publisher_id, subscriber_a, subscriber_b) = {
        let mut clients = clients.lock().await;

        let publisher_id = clients.register(publisher_sender);
        let subscriber_a = clients.register(sender_a);
        let subscriber_b = clients.register(sender_b);

        (publisher_id, subscriber_a, subscriber_b)
    };

    {
        let mut channels = channels.lock().await;

        channels
            .publish("game-123", publisher_id)
            .expect("publisher should be registered");

        channels.subscribe("game-123", subscriber_a);
        channels.subscribe("game-123", subscriber_b);
    }

    let msg = message(MessageType::Data, "game-123", b"frame-1");

    handle_message(&msg, publisher_id, &channels, &clients).await;

    let received_a = receiver_a
        .recv()
        .await
        .expect("subscriber A should receive DATA");

    let received_b = receiver_b
        .recv()
        .await
        .expect("subscriber B should receive DATA");

    assert_eq!(received_a.payload, b"frame-1");
    assert_eq!(received_b.payload, b"frame-1");
}

#[tokio::test]
async fn test_non_publisher_cannot_send_data() {
    let channels = Mutex::new(ChannelManager::new());
    let clients = Mutex::new(ClientManager::new());

    let (publisher_sender, _) = mpsc::channel(10);
    let (subscriber_sender, mut subscriber_receiver) = mpsc::channel(10);

    let (publisher_id, subscriber_id) = {
        let mut clients = clients.lock().await;

        let publisher_id = clients.register(publisher_sender);
        let subscriber_id = clients.register(subscriber_sender);

        (publisher_id, subscriber_id)
    };

    {
        let mut channels = channels.lock().await;

        channels
            .publish("game-123", publisher_id)
            .expect("publisher should be registered");

        channels.subscribe("game-123", subscriber_id);
    }

    let msg = message(MessageType::Data, "game-123", b"should-not-forward");

    handle_message(&msg, subscriber_id, &channels, &clients).await;

    assert!(
        subscriber_receiver.try_recv().is_err(),
        "non-publisher DATA should not be forwarded"
    );
}

#[tokio::test]
async fn test_data_unknown_channel_is_not_forwarded() {
    let channels = Mutex::new(ChannelManager::new());
    let clients = Mutex::new(ClientManager::new());

    let (publisher_sender, _) = mpsc::channel(10);
    let (subscriber_sender, mut subscriber_receiver) = mpsc::channel(10);

    let (publisher_id, subscriber_id) = {
        let mut clients = clients.lock().await;

        let publisher_id = clients.register(publisher_sender);
        let subscriber_id = clients.register(subscriber_sender);

        (publisher_id, subscriber_id)
    };

    {
        let mut channels = channels.lock().await;

        channels.subscribe("game-123", subscriber_id);
    }

    let msg = message(MessageType::Data, "does-not-exist", b"data");

    handle_message(&msg, publisher_id, &channels, &clients).await;

    assert!(
        subscriber_receiver.try_recv().is_err(),
        "DATA for unknown channel should not be forwarded"
    );
}
