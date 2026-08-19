use std::io;

use quinn::{RecvStream, SendStream};

pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1 MiB

pub async fn write_message(
    send: &mut SendStream,
    payload: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    if payload.len() > MAX_MESSAGE_SIZE {
        return Err("message exceeds maximum size".into());
    }

    let length = u32::try_from(payload.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "message too large"))?;

    send.write_all(&length.to_be_bytes())
        .await
        .expect("failed to write message length");
    send.write_all(payload)
        .await
        .expect("failed to write message payload");

    Ok(())
}

pub async fn read_message(
    recv: &mut RecvStream,
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut length_bytes = [0u8; 4];

    match recv.read_exact(&mut length_bytes).await {
        Ok(()) => {}
        Err(quinn::ReadExactError::FinishedEarly(_)) => {
            return Ok(None);
        }
        Err(error) => {
            return Err(error.into());
        }
    }

    let length = u32::from_be_bytes(length_bytes) as usize;

    if length > MAX_MESSAGE_SIZE {
        return Err("message exceeds maximum size".into());
    }

    let mut payload = vec![0u8; length];

    recv.read_exact(&mut payload).await?;

    Ok(Some(payload))
}
