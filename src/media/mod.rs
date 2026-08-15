use std::time::Duration;

pub mod encoder;

pub struct VideoFrame {
    pub timestamp: Option<Duration>,
    pub duration: Option<Duration>,
    pub data: Vec<u8>,
}

pub struct EncodedVideoFrame {
    pub timestamp: Option<Duration>,
    pub duration: Option<Duration>,
    pub keyframe: bool,
    pub data: Vec<u8>,
}
