use std::sync::Mutex;
use bytes::Bytes;

#[derive(Clone)]
struct Variables {
    pub RECORD_ACTIONS: bool,
    pub END_RECORD_BYTES: Bytes
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            RECORD_ACTIONS: false,
            END_RECORD_BYTES: Bytes::from("end record"),
        }
    }
}

static VARIABLES: Mutex<Option<Variables>> = Mutex::new(None);

pub fn end_record_bytes() -> Bytes {
    VARIABLES.lock().unwrap().clone().unwrap().END_RECORD_BYTES.clone()
}

pub fn start_recording_actions() {
    VARIABLES.lock().unwrap().clone().unwrap().RECORD_ACTIONS = true;
}

pub fn stop_recording_actions() {
    VARIABLES.lock().unwrap().clone().unwrap().RECORD_ACTIONS = false;
}

pub fn record_action(action: &str) {
    if VARIABLES.lock().unwrap().clone().unwrap().RECORD_ACTIONS {
        tracing::info!(action);
    }
}

pub fn is_recording_actions() -> bool {
    VARIABLES.lock().unwrap().clone().unwrap().RECORD_ACTIONS
}

pub fn init() {
    let mut variables = VARIABLES.lock().unwrap();
    if variables.is_some() {
        return;
    }
    *variables = Some(Variables::default());
    tracing::info!("Debug module initialized");
}