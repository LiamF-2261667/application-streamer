use crate::{FFmpegStream, Xvfb};

pub fn new(fps: u32, xvfb: &Xvfb) -> FFmpegStream {
    FFmpegStream::new(vec![
        "-hide_banner",
        "-v", "quiet",
        // "-stream_loop", "-1",
        "-y",
        "-r", &fps.to_string(),
        // "-c:v", "h264_nvenc",
        // "-re",
        "-f", "x11grab",
        "-s", &format!("{}x{}", xvfb.resolution().width, xvfb.resolution().height),
        "-i", &format!(":{}", xvfb.display()),
        "-preset", "ultrafast",
        "-crf", "28",
        // "-c", "copy",
        "-f", "mp4",
        "-movflags", "cmaf+separate_moof+delay_moov+skip_trailer+frag_every_frame",
        "pipe:1"
    ])
}