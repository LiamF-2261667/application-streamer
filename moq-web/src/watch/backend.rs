use moq_karp::{BroadcastConsumer, Input};
use wasm_bindgen_futures::spawn_local;
use moq_async::Lock;
use super::{ControlsRecv, Renderer, StatusSend, Video};
use crate::{Connect, ConnectionStatus, Error, Result};

struct BackendState {
	connect: Option<Connect>,
	broadcast: Option<BroadcastConsumer>,
	video: Option<Video>,
}

#[derive(Clone)]
pub struct Backend {
	controls: ControlsRecv,
	status: StatusSend,

	state: Lock<BackendState>,

	renderer: Renderer,
}

impl Backend {
	pub fn new(controls: ControlsRecv, status: StatusSend) -> Self {
		let state = Lock::new(BackendState {
			connect: None,
			broadcast: None,
			video: None,
		});

		Self {
			renderer: Renderer::new(controls.clone(), status.clone()),

			controls,
			status,

			state,
		}
	}

	pub fn start(mut self) {
		spawn_local(async move {
			if let Err(err) = self.run().await {
				tracing::error!(?err, "backend error");
				self.status.error.set(Some(err));
			}
		});
	}

	pub fn input(&mut self, input: Input) {
		let mut state = self.state.lock();
		if let Some(broadcast) = &mut state.broadcast {
			broadcast.input(input);
		}
	}

	async fn run(&mut self) -> Result<()> {
		loop {
			let mut state = self.state.lock();

			tokio::select! {
				url = self.controls.url.next() => {
					let url = url.ok_or(Error::Closed)?;

					state.broadcast = None;
					state.video = None;

					if let Some(url) = url {
						state.connect = Some(Connect::new(url));
						self.status.connection.update(ConnectionStatus::Connecting);
					} else {
						state.connect = None;
						self.status.connection.update(ConnectionStatus::Disconnected);
					}
				},
				Some(session) = async { Some(state.connect.as_mut()?.established().await) } => {
					let path = state.connect.take().unwrap().path;

					tracing::info!(?path, "Connected, loading broadcast");
					let broadcast = moq_karp::BroadcastConsumer::new(session?, path);
					self.status.connection.update(ConnectionStatus::Connected);

					state.broadcast = Some(broadcast);
					state.connect = None;
				},
				Some(catalog) = async { Some(state.broadcast.as_mut()?.next_catalog().await) } => {
					let catalog = match catalog? {
						Some(catalog) => {
							self.status.connection.update(ConnectionStatus::Live);
							catalog.clone()
						},
						None => {
							// There's no catalog, so the stream is offline.
							// Note: We keep trying because the stream might come online later.
							self.status.connection.update(ConnectionStatus::Offline);
							state.video = None;
							continue;
						},
					};

					// TODO add an ABR module
					if let Some(info) = catalog.video.first() {
						tracing::info!(?info, "Loading video track");

						let mut track = state.broadcast.as_mut().unwrap().track(&info.track)?;
						track.set_latency(self.controls.latency.get());
						self.renderer.set_resolution(info.resolution);

						let video = Video::new(track, info.clone())?;
						state.video = Some(video);
					} else {
						tracing::info!("No video track found");

						self.renderer.set_resolution(Default::default());
						state.video = None;
					}

				},
				Some(frame) = async { state.video.as_mut()?.frame().await.transpose() } => {
					self.renderer.push(frame?);
				},
				_ = self.controls.paused.next() => {
					// TODO temporarily unsubscribe on pause
				},
				latency = self.controls.latency.next() => {
					let latency = latency.ok_or(Error::Closed)?;
					if let Some(video) = state.video.as_mut() {
						 video.track.set_latency(latency);
					}
				},
				else => return Ok(()),
			}
		}
	}

	// async fn handle_url_change(mut self) -> Result<()> {
	// 	loop {
	// 		let mut state = self.state.lock();
	//
	// 		let url = self.controls.url.next().await;
	// 		let url = url.ok_or(Error::Closed)?;
	//
	// 		state.broadcast = None;
	// 		state.video = None;
	//
	// 		if let Some(url) = url {
	// 			state.connect = Some(Connect::new(url));
	// 			self.status.connection.update(ConnectionStatus::Connecting);
	// 		} else {
	// 			state.connect = None;
	// 			self.status.connection.update(ConnectionStatus::Disconnected);
	// 		}
	// 	}
	// }
}
