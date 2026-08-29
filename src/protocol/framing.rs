use crate::protocol::message::Message;
use quinn::{RecvStream, SendStream};

pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024;
const HEADER_SIZE: usize = 4;

// Encode a message as: [4-byte big-endian length][payload]
pub fn encode_message(message: &[u8]) -> Result<Vec<u8>, &'static str> {
    if message.len() > MAX_MESSAGE_SIZE {
        return Err("message exceeds maximum size");
    }
    let length = message.len() as u32;
    let mut frame = Vec::with_capacity(HEADER_SIZE + message.len());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(message);
    Ok(frame)
}

// Decode one complete frame from bytes.
pub fn decode_message(frame: &[u8]) -> Result<Vec<u8>, &'static str> {
    if frame.len() < HEADER_SIZE {
        return Err("frame is smaller than header");
    }
    let length = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
    if length > MAX_MESSAGE_SIZE {
        return Err("message exceeds maximum size");
    }
    if frame.len() != HEADER_SIZE + length {
        return Err("frame length does not match payload length");
    }
    Ok(frame[HEADER_SIZE..].to_vec())
}

// Write a framed message to a QUIC stream.
pub async fn write_message(
    send: &mut SendStream,
    message: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let frame = encode_message(message)?;
    send.write_all(&frame).await?;
    Ok(())
}

pub async fn write_protocol_message(
    send: &mut SendStream,
    message: &Message,
) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = message.encode()?;
    write_message(send, &encoded).await?;
    Ok(())
}

// Read one framed message from a QUIC stream.
pub async fn read_message(
    recv: &mut RecvStream,
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut header = [0u8; HEADER_SIZE];
    match recv.read_exact(&mut header).await {
        Ok(()) => {}
        Err(quinn::ReadExactError::FinishedEarly(_)) => {
            return Ok(None);
        }
        Err(error) => {
            return Err(error.into());
        }
    }
    let length = u32::from_be_bytes(header) as usize;
    if length > MAX_MESSAGE_SIZE {
        return Err("message exceeds maximum size".into());
    }
    let mut payload = vec![0u8; length];
    recv.read_exact(&mut payload).await?;
    Ok(Some(payload))
}

pub async fn read_protocol_message(
    recv: &mut RecvStream,
) -> Result<Option<Message>, Box<dyn std::error::Error>> {
    let bytes = match read_message(recv).await? {
        Some(bytes) => bytes,
        None => return Ok(None),
    };
    let message = Message::decode(&bytes)?;
    Ok(Some(message))
}
