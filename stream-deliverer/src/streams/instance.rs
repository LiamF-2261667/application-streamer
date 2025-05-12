#[derive(clone, Debug)]
pub struct StreamOptions {}

#[derive(clone, Debug)]
pub struct StreamInstance {
	port: u32,
	options: StreamOptions,
	container: Option<Container>,
}

impl StreamInstance {
	pub fn new(port: u32, options: StreamOptions) -> Self {
		StreamInstance {
			port: 0,
			options,
			container: None,
		}
	}

	pub fn start(&mut self) {
		// Logic to start the stream
	}

	pub fn stop(&mut self) {
		// Logic to stop the stream
	}
}
