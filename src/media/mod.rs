use gstreamer as gst;

pub mod encoder;
pub mod h264;
pub mod rav1e;

pub struct EncodedVideoUnit {
    pub timestamp: Option<gst::ClockTime>,
    pub duration: Option<gst::ClockTime>,
    pub keyframe: bool,
    pub data: Vec<u8>,
}
