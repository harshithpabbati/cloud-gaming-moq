use super::EncodedVideoUnit;

#[derive(Clone, Debug)]
pub struct VideoConfig {
    pub width: i32,
    pub height: i32,
    pub fps: i32,
    pub bitrate: i32,
}

pub trait VideoEncoder {
    type Error;

    fn encode(&mut self) -> Result<Vec<EncodedVideoUnit>, Self::Error>;
}
