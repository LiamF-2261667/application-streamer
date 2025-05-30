use crate::broadcast::InputHandlerRecv;
use crate::cmaf::Import;
use crate::fingerprint::FingerprintServer;
use crate::BroadcastProducer;
use anyhow::Context;
use moq_native::quic;
use moq_native::quic::Server;
use moq_transfork::web_transport;
use std::net::SocketAddr;
use tokio::io::AsyncRead;
use url::Url;

pub struct BroadcastServer<T: AsyncRead + Unpin> {
	bind: SocketAddr,
	tls: moq_native::tls::Args,
	input: T,
	broadcast: BroadcastProducer,
}

impl<T: AsyncRead + Unpin> BroadcastServer<T> {
	pub fn new(bind: SocketAddr, tls: moq_native::tls::Args, url: String, input: T) -> Self {
		// Create the broadcast
		let parsed_url = Url::parse(&url).context("invalid URL").expect("invalid URL");
		let path = parsed_url.path().to_string();
		let broadcast = BroadcastProducer::new(path).expect("failed to create broadcast");

		Self {
			bind,
			tls,
			input,
			broadcast,
		}
	}

	/// Runs the server.
	pub async fn run(&mut self) -> anyhow::Result<()> {
		self.bind = tokio::net::lookup_host(self.bind)
			.await
			.context("invalid bind address")?
			.next()
			.context("invalid bind address")?;

		let tls = self.tls.load()?;
		if tls.server.is_none() {
			anyhow::bail!("missing TLS certificates");
		}

		let quic = quic::Endpoint::new(quic::Config {
			bind: self.bind,
			tls: tls.clone(),
		})?;
		let server = quic.server.context("missing TLS certificate")?;

		// Create a web server to serve the fingerprint.
		let web = FingerprintServer::new(self.bind, tls);
		tokio::spawn(async move {
			web.run().await.expect("failed to run web server");
		});

		// Initialize the broadcast
		let mut import = Import::new(self.broadcast.clone());
		import
			.init_from(&mut self.input)
			.await
			.context("failed to initialize cmaf from input")?;

		self.accept(server)?;
		import.read_from(&mut self.input).await?; // Blocking method

		Ok(())
	}

	/// Returns a receiver for the input buffer.
	pub fn input_buffer(&self) -> InputHandlerRecv {
		self.broadcast.input_buffer()
	}

	fn accept(&mut self, mut server: Server) -> anyhow::Result<()> {
		tracing::info!(addr = %self.bind, "listening");

		// IMPORTANT! The line is used as a single for hosts that the application streamer is ready inside the container.
		// Keep it as is.
		println!("Ready to accept connections");

		let mut conn_id = 0;
		let mut broadcast = self.broadcast.clone();

		tokio::spawn(async move {
			while let Some(conn) = server.accept().await {
				// Create a new connection
				let session: web_transport::Session = conn.into();
				let transfork_session = moq_transfork::Session::accept(session)
					.await
					.expect("failed to accept session");

				conn_id += 1;
				broadcast.add_session(transfork_session).expect("failed to add session");

				tracing::info!(id = conn_id.clone(), "accepted");
			}
		});

		Ok(())
	}
}
