use std::env;
use std::thread::sleep;
use application_streamer::{stream, FFmpegStream, InputWriter, MoQInputStreamer, Xvfb, XvfbUser};
use moq_karp::{debug, Input, InputHandlerRecv};

const RESOLUTION: moq_karp::Dimensions = moq_karp::Dimensions { width: 1920, height: 1080 };
const PORT: u16 = 4443;
const FPS: u32 = 30;
const DISPLAY: u32 = 99;
// const TEST_VIDEO_FILE_LOCATION: &str = "C:/Users/liamf/Documents/Bach3/Bachelorproef/application-streamer/dev/bbb.fmp4";
const TEST_VIDEO_FILE_LOCATION: &str = "C:/AAA_Liam/School/Bach3/Bachelorproef/application-streamer/dev/bbb.fmp4";

// /// Stream video file with moq-karp
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
// 	let mut video_stream = stream::video_file::new(TEST_VIDEO_FILE_LOCATION);
// 	video_stream.start();
//
// 	let mut input_streamer = MoQInputStreamer::new(PORT, video_stream.stdout());
// 	let input_buffer = input_streamer.input_buffer();
// 	tokio::spawn(handle_input(input_buffer));
//
// 	input_streamer.stream().await?; // blocking method
//
// 	video_stream.stop().await;
//
// 	Ok(())
// }

///Stream xvfb with moq-karp
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	debug::init();

	// always remove the first argument
	let args: Vec<String> = env::args().skip(1).collect();
	let command = match args.len() {
		0 => "google-chrome --no-sandbox https://www.w3schools.com/html/mov_bbb.mp4",
		_ => &args.join(" ")
	};

	let mut xvfb = Xvfb::new(RESOLUTION, DISPLAY);
	let mut application = XvfbUser::new(&xvfb, command);
	let mut display_stream = stream::xvfb::new(FPS, &xvfb);

	xvfb.start();
	sleep(std::time::Duration::from_secs(1));
	application.start();
	sleep(std::time::Duration::from_secs(1));
	display_stream.start();
	sleep(std::time::Duration::from_secs(1));

	let mut input_streamer = MoQInputStreamer::new(PORT, display_stream.stdout());
	let input_buffer = input_streamer.input_buffer();
	tokio::spawn(async move {
		handle_input(input_buffer, application).await;
	});
	input_streamer.stream().await?; // blocking method

	display_stream.stop().await;
	xvfb.stop().await;

	Ok(())
}

async fn handle_input(mut input_buffer: InputHandlerRecv, application: XvfbUser) {
	loop {
		match input_buffer.input.next().await {
			Some(input) => {
				let input = input.expect("failed to read input");
				application.handle(input.clone());

				// DEBUG: start recording actions on space key press
				if let Input::KeyDown(key) = input {
					if key.name().to_lowercase() == "space" {
						debug::start_recording_actions();
					}
				}
			}
			None => { }
		}
	}
}