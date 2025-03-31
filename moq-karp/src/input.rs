use std::fmt::Display;
use bytes::Bytes;
use serde_with::formats::Format;

#[derive(Clone)]
pub struct Key {
    name: String
}
impl Key {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub enum Input {
    KeyDown(Key),
    KeyUp(Key),
    MouseMove(i32, i32),
}

impl Into<Bytes> for Input {
    fn into(self) -> Bytes {
        match self {
            Input::KeyDown(key) => format!("keydown:{}", key.name()).into(),
            Input::KeyUp(key) => format!("keyup:{}", key.name()).into(),
            Input::MouseMove(x, y) => format!("mousemove:{},{}", x, y).into(),
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::KeyDown(key) => write!(f, "keydown:{}", key.name()),
            Input::KeyUp(key) => write!(f, "keyup:{}", key.name()),
            Input::MouseMove(x, y) => write!(f, "mousemove:{},{}", x, y),
        }
    }
}