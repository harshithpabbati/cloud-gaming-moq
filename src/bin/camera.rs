use gst::prelude::*;
use gstreamer as gst;
use gstreamer_app as gst_app;

const VIDEO_WIDTH: i32 = 1280;
const VIDEO_HEIGHT: i32 = 720;
const VIDEO_FPS: i32 = 30;

fn main() {
    gst::init().expect("failed to initialize GStreamer");

    let pipeline = gst::Pipeline::new();

    let camera = gst::ElementFactory::make("avfvideosrc")
        .build()
        .expect("failed to create camera source");

    let camera_caps = gst::Caps::builder("video/x-raw")
        .field("width", VIDEO_WIDTH)
        .field("height", VIDEO_HEIGHT)
        .field("framerate", gst::Fraction::new(VIDEO_FPS, 1))
        .build();

    let caps_filter = gst::ElementFactory::make("capsfilter")
        .property("caps", &camera_caps)
        .build()
        .expect("failed to create caps filter");

    let converter = gst::ElementFactory::make("videoconvert")
        .build()
        .expect("failed to create video converter");

    let encoder = gst::ElementFactory::make("rav1enc")
        .property("bitrate", 2_000_000i32)
        .property("low-latency", true)
        .property("max-key-frame-interval", 60u64)
        .property("min-key-frame-interval", 12u64)
        .property("speed-preset", 8u32)
        .build()
        .expect("failed to create AV1 encoder");

    let appsink = gst_app::AppSink::builder().build();

    pipeline
        .add_many([
            &camera,
            &caps_filter,
            &converter,
            &encoder,
            appsink.upcast_ref(),
        ])
        .expect("failed to add elements to pipeline");

    camera
        .link(&caps_filter)
        .expect("failed to link camera to caps filter");

    caps_filter
        .link(&converter)
        .expect("failed to link caps filter to converter");

    converter
        .link(&encoder)
        .expect("failed to link converter to encoder");

    encoder
        .link(&appsink)
        .expect("failed to link encoder to appsink");

    pipeline
        .set_state(gst::State::Playing)
        .expect("failed to start pipeline");

    println!("Camera → AV1 pipeline is running");

    loop {
        let sample = appsink.pull_sample().expect("failed to pull sample");

        let buffer = sample.buffer().expect("sample has no buffer");

        let caps = sample.caps().expect("sample has no caps");

        let map = buffer.map_readable().expect("failed to map buffer");

        let bytes = map.as_slice();
        let preview_len = bytes.len().min(16);

        let is_keyframe = !buffer.flags().contains(gst::BufferFlags::DELTA_UNIT);

        println!("Encoded unit:");
        println!("  size: {} bytes", bytes.len());
        println!("  keyframe: {is_keyframe}");
        println!("  pts: {:?}", buffer.pts());
        println!("  duration: {:?}", buffer.duration());
        println!("  first bytes: {:02x?}", &bytes[..preview_len]);
        println!("  caps: {caps}");
    }
}
