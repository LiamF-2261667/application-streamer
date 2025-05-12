use moq_async::Lock;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct StreamsManager {
	streams: Lock<HashMap<u32, StreamInstance>>,
}

#[wasm_bindgen]
impl StreamsManager {
	#[wasm_bindgen(constructor)]
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
