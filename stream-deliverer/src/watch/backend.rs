use super::{ControlsRecv, InputsRecv, Renderer, StatusSend, Video};
use crate::{Connect, ConnectionStatus, Error, Result};
use moq_karp::{BroadcastConsumer, Input};
use wasm_bindgen_futures::spawn_local;

pub struct Backend {
	controls: ControlsRecv,
	inputs: InputsRecv,
	status: StatusSend,

	connect: Option<Connect>,
	broadcast: Option<BroadcastConsumer>,
	video: Option<Video>,

	renderer: Renderer,
}

impl Backend {
	pub fn new(controls: ControlsRecv, inputs: InputsRecv, status: StatusSend) -> Self {
		Self {
			renderer: Renderer::new(controls.clone(), status.clone()),

			controls,
			inputs,
			status,

			connect: None,
			broadcast: None,
			video: None,
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

	async fn run(&mut self) -> Result<()> {
		loop {
			tokio::select! {
				input = self.inputs.input.next() => {
					if let Some(broadcast) = &mut self.broadcast {
						let actual_input = input.ok_or("closed").expect("input").expect("input");
						let _ = broadcast.input(actual_input.clone());

						// DEBUG: log when space key is pressed
						if let Input::KeyDown(keydown) = actual_input {
							if keydown.name() == "space" || keydown.name() == " " {
								tracing::info!("Space key press written to track");
								println!("[println!] Space key press written to track!")
							}
						}
					}
				}
				url = self.controls.url.next() => {
					let url = url.ok_or(Error::Closed)?;

					self.broadcast = None;
					self.video = None;

					if let Some(url) = url {
						self.connect = Some(Connect::new(url));
						self.status.connection.update(ConnectionStatus::Connecting);
					} else {
						self.connect = None;
						self.status.connection.update(ConnectionStatus::Disconnected);
					}
				},
				Some(session) = async { Some(self.connect.as_mut()?.established().await) } => {
					let path = self.connect.take().unwrap().path;

					tracing::info!(?path, "Connected, loading broadcast");
					let broadcast = moq_karp::BroadcastConsumer::new(session?, path);
					self.status.connection.update(ConnectionStatus::Connected);

					self.broadcast = Some(broadcast);
					self.connect = None;
				},
				Some(catalog) = async { Some(self.broadcast.as_mut()?.next_catalog().await) } => {
					let catalog = match catalog? {
						Some(catalog) => {
							self.status.connection.update(ConnectionStatus::Live);
							catalog.clone()
						},
						None => {
							// There's no catalog, so the stream is offline.
							// Note: We keep trying because the stream might come online later.
							self.status.connection.update(ConnectionStatus::Offline);
							self.video = None;
							continue;
						},
					};

					// TODO add an ABR module
					if let Some(info) = catalog.video.first() {
						tracing::info!(?info, "Loading video track");

						let mut track = self.broadcast.as_mut().unwrap().track(&info.track)?;
						track.set_latency(self.controls.latency.get());
						self.renderer.set_resolution(info.resolution);

						let video = Video::new(track, info.clone())?;
						self.video = Some(video);
					} else {
						tracing::info!("No video track found");

						self.renderer.set_resolution(Default::default());
						self.video = None;
					}

				},
				Some(frame) = async { self.video.as_mut()?.frame().await.transpose() } => {
					self.renderer.push(frame?);
				},
				_ = self.controls.paused.next() => {
					// TODO temporarily unsubscribe on pause
				},
				latency = self.controls.latency.next() => {
					let latency = latency.ok_or(Error::Closed)?;
					if let Some(video) = self.video.as_mut() {
						 video.track.set_latency(latency);
					}
				},
				else => return Ok(()),
			}
		}
	}
}
