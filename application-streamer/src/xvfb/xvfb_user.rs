use moq_karp::Input;
use crate::xvfb::Xvfb;

pub struct XvfbUser {
    xvfb_display: u32,
    start_cmd: String,
}

impl XvfbUser {
    pub fn new(xvfb: &Xvfb, start_cmd: &str) -> Self {
        Self { xvfb_display: xvfb.display(), start_cmd: start_cmd.to_string() }
    }

    /// Starts the xvfb user.
    pub fn start(&mut self) {
        set_display_var(self.xvfb_display);

        let start_cmd = self.start_cmd.clone();

        tokio::spawn(async move {
            execute(&start_cmd);
        });
    }

    /// Sends an input to the xvfb user.
    pub fn handle(&self, input: Input) {
        let cmd = input_to_cmd(input);

        tokio::spawn(async move {
            execute(&cmd);
        });
    }
}

fn execute(cmd: &str) {
    tokio::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("failed to execute command");
}

fn input_to_cmd(input: Input) -> String {
    let base_cmd = "xdotool ";
    match input {
        Input::KeyDown(key) => format!("{} keydown {}", base_cmd, key.name()),
        Input::KeyUp(key) => format!("{} keyup {}", base_cmd, key.name()),
        Input::MouseMove(x, y) => format!("{} mousemove {} {}", base_cmd, x, y),
    }
}

fn set_display_var(display: u32) {
    unsafe {
        std::env::set_var("DISPLAY", format!(":{}", display));
    }
    assert_eq!(std::env::var("DISPLAY").unwrap(), format!(":{}", display), "failed to set DISPLAY variable");
}