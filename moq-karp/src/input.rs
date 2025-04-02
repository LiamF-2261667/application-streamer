use std::fmt::Display;
use bytes::Bytes;

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
    MouseDown(i32),
    MouseUp(i32),
}

impl Into<Bytes> for Input {
    fn into(self) -> Bytes {
        match self {
            Input::KeyDown(key) => format!("keydown:{}", key.name()).into(),
            Input::KeyUp(key) => format!("keyup:{}", key.name()).into(),
            Input::MouseMove(x, y) => format!("mousemove:{},{}", x, y).into(),
            Input::MouseDown(button) => format!("mousedown:{}", button).into(),
            Input::MouseUp(button) => format!("mouseup:{}", button).into(),
        }
    }
}

impl Into<Input> for Bytes {
    fn into(self) -> Input {
        let input = String::from_utf8(self.to_vec()).unwrap();
        let mut parts = input.split(':');
        let action = parts.next().unwrap();
        let value = parts.next().unwrap();

        match action {
            "keydown" => Input::KeyDown(Key::new(value)),
            "keyup" => Input::KeyUp(Key::new(value)),
            "mousemove" => {
                let mut parts = value.split(',');
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                Input::MouseMove(x, y)
            },
            "mousedown" => Input::MouseDown(value.parse().unwrap()),
            "mouseup" => Input::MouseUp(value.parse().unwrap()),
            _ => panic!("invalid input: {}", input)
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::KeyDown(key) => write!(f, "keydown:{}", key.name()),
            Input::KeyUp(key) => write!(f, "keyup:{}", key.name()),
            Input::MouseMove(x, y) => write!(f, "mousemove:{},{}", x, y),
            Input::MouseDown(button) => write!(f, "mousedown:{}", button),
            Input::MouseUp(button) => write!(f, "mouseup:{}", button),
        }
    }
}