use std::sync::atomic::AtomicBool;
use bytes::Bytes;

use once_cell::sync::Lazy;

static RECORD_ACTIONS: AtomicBool = AtomicBool::new(false);

pub fn start_recording_actions() {
    RECORD_ACTIONS.store(true, std::sync::atomic::Ordering::SeqCst);
}

pub fn stop_recording_actions() {
    RECORD_ACTIONS.store(false, std::sync::atomic::Ordering::SeqCst);
}

pub fn record_action(action: &str) {
    if is_recording_actions() {
        tracing::info!(action);
    }
}

pub fn is_recording_actions() -> bool {
    RECORD_ACTIONS.load(std::sync::atomic::Ordering::SeqCst)
}