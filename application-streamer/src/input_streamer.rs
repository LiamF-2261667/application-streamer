use moq_karp::{BroadcastServer, InputHandlerRecv};
use std::net;
use std::str::FromStr;
use tokio::io::AsyncRead;

pub struct MoQInputStreamer<T: AsyncRead + Unpin> {
	log: moq_native::log::Args,
	server: BroadcastServer<T>,
}

impl<T: AsyncRead + Unpin> MoQInputStreamer<T> {
	pub fn new(port: u16, input: T) -> Self {
		let mut tls = moq_native::tls::Args::default();
		tls.self_sign.push(String::from("localhost:4443"));
		tls.disable_verify = true;

		let bind = net::SocketAddr::from_str(format!("[::]:{}", port).as_str()).unwrap();

		let server = BroadcastServer::new(bind, tls, String::from(format!("http://localhost:{}/", port)), input);

		Self {
			log: moq_native::log::Args::default(),
			server,
		}
	}

	pub fn input_buffer(&self) -> InputHandlerRecv {
		self.server.input_buffer()
	}

	pub async fn stream(&mut self) -> anyhow::Result<()> {
		self.log.init();
		self.server.run().await
	}
}
