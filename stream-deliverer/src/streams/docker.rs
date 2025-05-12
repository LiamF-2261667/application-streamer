use std::fmt::Display;
use moq_async::Lock;

#[derive(Clone)]
pub struct Tag {
	pub name: String,
	pub version: String,
}

impl Display for Tag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.name, self.version)
	}
}

#[derive(Clone)]
pub struct DockerFile {
	location: String,
	tag: Tag,
	context: String,
}
impl DockerFile {
	pub fn new(location: &str, tag: Tag, context: &str) -> Self {
		DockerFile {
			location: location.to_string(),
			tag,
			context: context.to_string(),
		}
	}

	pub fn get_location(&self) -> Location {
		self.location.clone()
	}
	pub fn get_tag(&self) -> Tag {
		self.tag.clone()
	}
	pub fn get_context(&self) -> Location {
		self.context.clone()
	}

	pub async fn build(&self) -> Image {
		const CMD: &str = format!(
			"docker build -t {} -f {} {}",
			self.tag.name, self.location, self.context
		)
		.as_str();

		tokio::process::Command::new(CMD)
			.await
			.expect("Failed to build docker image");

		Image::new(self.tag.clone())
	}
}

#[derive(Clone)]
pub struct Image {
	tag: Tag,
}
impl Image {
	pub fn new(tag: Tag) -> Self {
		Image { tag }
	}

	pub fn get_tag(&self) -> Tag {
		self.tag.clone()
	}
}

#[derive(Clone)]
pub enum Status {
	Running,
	Stopped,
	Exited,
}

impl PartialEq<Self> for Status {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Status::Running, Status::Running) => true,
			(Status::Stopped, Status::Stopped) => true,
			(Status::Exited, Status::Exited) => true,
			_ => false,
		}
	}
}

#[derive(Clone)]
pub struct Container {
	name: String,
	image: Image,
	ports: Vec<u16>,
	status: Lock<Status>,
}
impl Container {
	pub fn new(name: &str, image: Image) -> Self {
		Container {
			name: name.to_string(),
			image,
			ports: vec![],
			status: Lock::new(Status::Stopped),
		}
	}

	pub fn get_ports(&self) -> Vec<u16> {
		self.ports.clone()
	}
	pub fn get_name(&self) -> String {
		self.name.clone()
	}
	pub fn get_image(&self) -> Image {
		self.image.clone()
	}
	pub fn get_status(&self) -> Status {
		self.status.lock().clone()
	}

	pub fn add_port(&mut self, port: u16) {
		self.ports.push(port);
	}
	pub fn remove_port(&mut self, port: u16) {
		self.ports.retain(|&p| p != port);
	}

	pub async fn start(&self) -> Result<(), Err> {
		let mut status = self.status.lock();
		if *status == Status::Running {
			return Err("Container is already running".into());
		}
		*status = Status::Running;

		const CMD: &str = format!(
			"docker run --rm --name {} -p {} {}",
			self.name,
			self.ports
				.iter()
				.map(|port| format!("{}:{}", port, port))
				.collect::<Vec<String>>()
				.join(" "),
			self.image.get_tag()
		)
		.as_str();

		tokio::process::Command::new(CMD)
			.await
			.expect("Failed to start docker container");

		tracing::info!("Started container: {}", self.name);
	}

	pub async fn stop(&self) -> Result<(), Err> {
		let mut status = self.status.lock();
		if *status != Status::Running {
			return Err("Container is not running".into());
		}
		*status = Status::Stopped;

		const CMD: &str = format!("docker stop {}", self.name).as_str();
		tokio::process::Command::new(CMD)
			.await
			.expect("Failed to stop docker container");

		tracing::info!("Stopped container: {}", self.name);
	}
}
