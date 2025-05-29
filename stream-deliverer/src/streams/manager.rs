use moq_async::Lock;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use crate::streams::instance::StreamInstance;

pub struct StreamsManager {
	streams: Lock<HashMap<u32, StreamInstance>>,
}

impl StreamsManager {
	pub fn new() -> Self {
		StreamsManager {
			streams: Lock::new(HashMap::new()),
		}
	}

	pub fn start(&self, stream_id: &str) {
		web_sys::console::debug_1(&JsValue::from_str(stream_id));
	}

	pub fn stop(&self, stream_id: &str) {
		println!("stopping stream {}", stream_id);
		// Logic to stop the stream
	}
}
