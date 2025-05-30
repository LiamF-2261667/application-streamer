use std::time::Duration;

use baton::Baton;
use url::Url;
use wasm_bindgen::prelude::*;

use super::{Backend, StatusRecv, WatchStatus};
use crate::{Error, Result, watch::Status};
use moq_karp::{Input, Key};
use web_sys::OffscreenCanvas;

// Sent from the frontend to the backend.
#[derive(Debug, Baton)]
pub(super) struct Controls {
	pub url: Option<Url>,
	pub paused: bool,
	pub volume: f64,
	pub canvas: Option<web_sys::OffscreenCanvas>,

	// Play media faster until this latency is reached.
	pub latency: Duration,
}

#[derive(Default, Clone, Baton)]
pub(super) struct Inputs {
	pub input: Option<Input>,
}

impl Default for Controls {
	fn default() -> Self {
		Self {
			url: None,
			paused: false,
			volume: 1.0,
			canvas: None,
			latency: Duration::ZERO,
		}
	}
}
#[wasm_bindgen]
pub struct Watch {
	controls: ControlsSend,
	inputs: InputsSend,
	status: StatusRecv,
}

#[wasm_bindgen]
impl Watch {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		let controls = Controls::default().baton();
		let inputs = Inputs::default().baton();
		let status = Status::default().baton();

		let backend = Backend::new(controls.1, inputs.1, status.0);
		backend.start();

		Self {
			controls: controls.0,
			inputs: inputs.0,
			status: status.1,
		}
	}

	pub fn url(&mut self, url: Option<String>) -> Result<()> {
		let url = match url {
			Some(url) => Url::parse(&url).map_err(|_| Error::InvalidUrl(url.to_string()))?.into(),
			None => None,
		};
		self.controls.url.set(url);
		Ok(())
	}

	pub fn paused(&mut self, paused: bool) {
		self.controls.paused.set(paused);
	}

	pub fn volume(&mut self, volume: f64) {
		self.controls.volume.set(volume);
	}

	pub fn canvas(&mut self, canvas: Option<OffscreenCanvas>) {
		self.controls.canvas.set(canvas);
	}

	pub fn latency(&mut self, latency: u32) {
		self.controls.latency.set(Duration::from_millis(latency as _));
	}

	pub fn status(&self) -> WatchStatus {
		WatchStatus::new(self.status.clone())
	}

	pub fn keydown(&mut self, key: String) {
		let input = Input::KeyDown(Key::new(key));
		self.inputs.input.set(Some(input));
	}
	pub fn keyup(&mut self, key: String) {
		let input = Input::KeyUp(Key::new(key));
		self.inputs.input.set(Some(input));
	}
	pub fn mousemove(&mut self, x: i32, y: i32, page_width: i32, page_height: i32) {
		// transform to stream coordinates
		// assuming the stream is 1920x1080
		// TO-DO: send x and y as percentages and multiply in the application-streamer instead
		let x = ((x as f64 / page_width as f64) * 1920.0) as i32;
		let y = ((y as f64 / page_height as f64) * 1080.0) as i32;

		let input = Input::MouseMove(x, y);
		self.inputs.input.set(Some(input));
	}
	pub fn mousedown(&mut self, button: i32) {
		let input = Input::MouseDown(button);
		self.inputs.input.set(Some(input));
	}
	pub fn mouseup(&mut self, button: i32) {
		let input = Input::MouseUp(button);
		self.inputs.input.set(Some(input));
	}
}

impl Default for Watch {
	fn default() -> Self {
		Self::new()
	}
}
