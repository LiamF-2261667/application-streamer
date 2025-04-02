use std::env;
use std::thread::sleep;
use application_streamer::{stream, FFmpegStream, InputWriter, MoQInputStreamer, Xvfb, XvfbUser};

const RESOLUTION: moq_karp::Dimensions = moq_karp::Dimensions { width: 1920, height: 1080 };
const PORT: u16 = 4443;
const FPS: u32 = 30;
const DISPLAY: u32 = 99;
const TEST_VIDEO_FILE_LOCATION: &str = "C:/Users/liamf/Documents/Bach3/Bachelorproef/application-streamer/dev/bbb.fmp4";

/// Stream video file with moq-karp
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut video_stream = stream::video_file::new(TEST_VIDEO_FILE_LOCATION);

	video_stream.start();

	let mut input_streamer = MoQInputStreamer::new(PORT);
	input_streamer.stream(video_stream.stdout()).await?; // blocking method

	video_stream.stop().await;

	Ok(())
}

// ///Stream xvfb with moq-karp
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
// 	// always remove the first argument
// 	let args: Vec<String> = env::args().skip(1).collect();
// 	let command = match args.len() {
// 		0 => "google-chrome --no-sandbox https://www.w3schools.com/html/mov_bbb.mp4",
// 		_ => &args.join(" ")
// 	};
//
// 	let mut xvfb = Xvfb::new(RESOLUTION, DISPLAY);
// 	let mut application = XvfbUser::new(&xvfb, command);
// 	let mut display_stream = stream::xvfb::new(FPS, &xvfb);
//
// 	xvfb.start();
// 	sleep(std::time::Duration::from_secs(1));
// 	application.start();
// 	sleep(std::time::Duration::from_secs(1));
// 	display_stream.start();
// 	sleep(std::time::Duration::from_secs(1));
//
// 	// let mut input_writer = InputWriter::new("/tmp/out_custom.mp4");
// 	// input_writer.write(display_stream.stdout()).await?; // blocking method
//
// 	let mut input_streamer = MoQInputStreamer::new(PORT);
// 	input_streamer.stream(display_stream.stdout()).await?; // blocking method
//
// 	display_stream.stop().await;
// 	application.stop().await;
// 	xvfb.stop().await;
//
// 	Ok(())
// }