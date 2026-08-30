use std::convert::TryFrom;

use super::MAX_FRAME_PAYLOAD_SIZE;

const MAX_CHANNEL_NAME_SIZE: usize = 256;
const MESSAGE_HEADER_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    Publish = 1,
    Subscribe = 2,
    Unsubscribe = 3,
    Data = 4,
}

impl TryFrom<u8> for MessageType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MessageType::Publish),
            2 => Ok(MessageType::Subscribe),
            3 => Ok(MessageType::Unsubscribe),
            4 => Ok(MessageType::Data),
            _ => Err("unknown message type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub message_type: MessageType,
    pub channel_name: String,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn encode(&self) -> Result<Vec<u8>, &'static str> {
        let channel = self.channel_name.as_bytes();
        if channel.len() > MAX_CHANNEL_NAME_SIZE {
            return Err("channel name exceeds maximum size");
        }
        if channel.len() > u16::MAX as usize {
            return Err("channel name is too long");
        }

        let encoded_len = MESSAGE_HEADER_SIZE
            .checked_add(channel.len())
            .and_then(|length| length.checked_add(self.payload.len()))
            .ok_or("message exceeds maximum framed size")?;
        if encoded_len > MAX_FRAME_PAYLOAD_SIZE {
            return Err("message exceeds maximum framed size");
        }

        let channel_len = channel.len() as u16;
        let mut encoded = Vec::with_capacity(encoded_len);
        encoded.push(self.message_type as u8);
        encoded.extend_from_slice(&channel_len.to_be_bytes());
        encoded.extend_from_slice(channel);
        encoded.extend_from_slice(&self.payload);
        Ok(encoded)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < MESSAGE_HEADER_SIZE {
            return Err("message is too short");
        }
        if bytes.len() > MAX_FRAME_PAYLOAD_SIZE {
            return Err("message exceeds maximum framed size");
        }
        let message_type = MessageType::try_from(bytes[0])?;
        let channel_len = u16::from_be_bytes([bytes[1], bytes[2]]) as usize;
        if channel_len > MAX_CHANNEL_NAME_SIZE {
            return Err("channel name exceeds maximum size");
        }

        let channel_start = 3;
        let channel_end = channel_start + channel_len;

        if bytes.len() < channel_end {
            return Err("message contains incomplete channel name");
        }

        let channel_name = String::from_utf8(bytes[channel_start..channel_end].to_vec())
            .map_err(|_| "channel name is not valid UTF-8")?;
        let payload = bytes[channel_end..].to_vec();
        Ok(Self {
            message_type,
            channel_name,
            payload,
        })
    }
}
