use gst::prelude::*;
use gstreamer as gst;
use gstreamer_app as gst_app;

use super::{
    EncodedVideoUnit,
    encoder::{VideoConfig, VideoEncoder},
};

pub struct Rav1eEncoder {
    appsink: gst_app::AppSink,
}

impl Rav1eEncoder {
    pub fn new(config: VideoConfig) -> Result<Self, gst::glib::BoolError> {
        let pipeline = gst::Pipeline::new();

        let camera = gst::ElementFactory::make("avfvideosrc").build()?;

        let camera_caps = gst::Caps::builder("video/x-raw")
            .field("width", config.width)
            .field("height", config.height)
            .field("framerate", gst::Fraction::new(config.fps, 1))
            .build();

        let caps_filter = gst::ElementFactory::make("capsfilter")
            .property("caps", &camera_caps)
            .build()?;

        let converter = gst::ElementFactory::make("videoconvert").build()?;

        let encoder = gst::ElementFactory::make("rav1enc")
            .property("bitrate", config.bitrate)
            .property("low-latency", true)
            .property("max-key-frame-interval", 60u64)
            .property("min-key-frame-interval", 12u64)
            .property("speed-preset", 8u32)
            .build()?;

        let appsink = gst_app::AppSink::builder().build();

        pipeline.add_many([
            &camera,
            &caps_filter,
            &converter,
            &encoder,
            appsink.upcast_ref(),
        ])?;

        camera.link(&caps_filter)?;
        caps_filter.link(&converter)?;
        converter.link(&encoder)?;
        encoder.link(&appsink)?;

        pipeline
            .set_state(gst::State::Playing)
            .map_err(|error| gst::glib::bool_error!("failed to start pipeline: {error}"))?;

        Ok(Self { appsink })
    }
}

impl VideoEncoder for Rav1eEncoder {
    type Error = gst::glib::BoolError;

    fn encode(&mut self) -> Result<Vec<EncodedVideoUnit>, Self::Error> {
        let sample = self.appsink.pull_sample()?;

        let buffer = sample.buffer().expect("sample has no buffer");

        let map = buffer.map_readable().expect("failed to map encoded buffer");

        let data = map.as_slice().to_vec();

        let unit = EncodedVideoUnit {
            timestamp: buffer.pts(),
            duration: buffer.duration(),
            keyframe: !buffer.flags().contains(gst::BufferFlags::DELTA_UNIT),
            data,
        };

        Ok(vec![unit])
    }
}
