use super::{EncodedVideoFrame, VideoFrame};

pub trait VideoEncoder {
    type Error;

    fn encode(&mut self, frame: VideoFrame) -> Result<Vec<EncodedVideoFrame>, Self::Error>;
}
