use moq_rs::protocol::framing::{MAX_MESSAGE_SIZE, decode_message, encode_message};

#[test]
fn test_encode_and_decode_message() {
    let message = b"Hello RelayMoQ";
    let encoded = encode_message(message).expect("failed to encode message");
    let decoded = decode_message(&encoded).expect("failed to decode message");
    assert_eq!(decoded, message);
}

#[test]
fn test_empty_message() {
    let message = b"";
    let encoded = encode_message(message).expect("failed to encode message");
    let decoded = decode_message(&encoded).expect("failed to decode message");
    assert_eq!(decoded, message);
}

#[test]
fn test_multiple_messages() {
    let messages: &[&[u8]] = &[b"Hello", b"World", b"MoQ"];
    for message in messages {
        let encoded = encode_message(message).expect("failed to encode message");
        let decoded = decode_message(&encoded).expect("failed to decode message");
        assert_eq!(decoded, *message);
    }
}

#[test]
fn test_oversized_message() {
    let message = vec![0u8; MAX_MESSAGE_SIZE + 1];

    let result = encode_message(&message);

    assert!(result.is_err());
}

#[test]
fn test_truncated_header() {
    let frame = [0u8, 0, 0];

    let result = decode_message(&frame);

    assert!(result.is_err());
}

#[test]
fn test_truncated_payload() {
    // Header says payload is 10 bytes,
    // but only 3 bytes are actually present.
    let mut frame = vec![0u8, 0, 0, 10];
    frame.extend_from_slice(b"abc");

    let result = decode_message(&frame);

    assert!(result.is_err());
}

#[test]
fn test_invalid_length() {
    let mut frame = vec![0u8, 0, 0, 5];
    frame.extend_from_slice(b"HelloWorld");

    let result = decode_message(&frame);

    assert!(result.is_err());
}

#[test]
fn test_frame_format() {
    let message = b"Hello";

    let encoded = encode_message(message).expect("failed to encode message");

    assert_eq!(&encoded[..4], &[0, 0, 0, 5]);

    assert_eq!(&encoded[4..], message);
}
