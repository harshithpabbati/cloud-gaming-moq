use moq_rs::protocol::message::{Message, MessageType};

#[test]
fn test_subscribe_message() {
    let message = Message {
        message_type: MessageType::Subscribe,
        channel_name: "game-123".to_string(),
        payload: Vec::new(),
    };

    let encoded = message.encode().expect("failed to encode message");

    let decoded = Message::decode(&encoded).expect("failed to decode message");

    assert_eq!(decoded, message);
}

#[test]
fn test_publish_message() {
    let message = Message {
        message_type: MessageType::Publish,
        channel_name: "game-123".to_string(),
        payload: b"Hello".to_vec(),
    };

    let encoded = message.encode().expect("failed to encode message");

    let decoded = Message::decode(&encoded).expect("failed to decode message");

    assert_eq!(decoded, message);
}

#[test]
fn test_data_message() {
    let message = Message {
        message_type: MessageType::Data,
        channel_name: "game-123".to_string(),
        payload: b"frame data".to_vec(),
    };

    let encoded = message.encode().expect("failed to encode message");

    let decoded = Message::decode(&encoded).expect("failed to decode message");

    assert_eq!(decoded, message);
}

#[test]
fn test_unknown_message_type() {
    let bytes = [99, 0, 4, b't', b'e', b's', b't'];

    let result = Message::decode(&bytes);

    assert!(result.is_err());
}
