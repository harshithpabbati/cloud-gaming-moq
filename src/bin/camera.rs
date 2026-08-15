use moq_rs::media::{
    encoder::{VideoConfig, VideoEncoder},
    rav1e::Rav1eEncoder,
};

fn main() {
    gstreamer::init().expect("failed to initialize GStreamer");

    let config = VideoConfig {
        width: 1280,
        height: 720,
        fps: 30,
        bitrate: 2_000_000,
    };

    let mut encoder = Rav1eEncoder::new(config).expect("failed to create AV1 encoder");

    println!("Camera → AV1 pipeline is running");

    loop {
        let units = encoder.encode().expect("failed to encode video");

        for unit in units {
            println!(
                "{} bytes | keyframe={} | pts={:?}",
                unit.data.len(),
                unit.keyframe,
                unit.timestamp
            );
        }
    }
}
