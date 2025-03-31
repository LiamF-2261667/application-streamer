use bytes::Bytes;

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