use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct StreamsManager {

}

#[wasm_bindgen]
impl StreamsManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        StreamsManager {}
    }

    pub fn start_stream(&self, stream_id: &str) {
        web_sys::console::debug_1(&JsValue::from_str(stream_id));
        // Logic to start the stream
    }

    pub fn stop_stream(&self, stream_id: &str) {
        println!("stopping stream {}", stream_id);
        // Logic to stop the stream
    }
}