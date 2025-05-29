import * as Comlink from "comlink";

// TODO investigate if enums are causing the WASM module to be loaded.
// Without enums, we could `import type` to avoid this?
import * as Rust from "@rust";
import { type ConnectionStatus, convertConnectionStatus } from "../connection";
import type { Bridge } from "./bridge";

import { MoqElement, attribute, element, jsx } from "../util";
import {apiRequest} from "../service";

export type RendererStatus = "idle" | "paused" | "buffering" | "live";

// Create a new worker instance that is shared between all instances of the Watch class.
// We wait until the worker is fully initialized before we return the proxy.
const worker: Promise<Comlink.Remote<Bridge>> = new Promise((resolve) => {
	const worker = new Worker(new URL("./bridge", import.meta.url), {
		type: "module",
	});
	worker.addEventListener(
		"message",
		(event) => {
			const proxy: Comlink.Remote<Bridge> = Comlink.wrap(worker);
			resolve(proxy);
		},
		{ once: true },
	);
});

@element("moq-watch")
export class Watch extends MoqElement {
	#worker: Promise<Comlink.Remote<Rust.Watch>>;
	#canvas: HTMLCanvasElement | null = null;

	@attribute
	accessor url = "";

	@attribute
	accessor paused = false;

	@attribute
	accessor volume = 1;

	@attribute
	accessor latency = 0;

	constructor() {
		super();

		this.#worker = worker.then((w) => w.watch());

		const style = (
			<style>
				{`
				:host {
					display: block;
					position: relative;
					overflow: hidden;

					max-width: 100%;
					max-height: 100%;

					justify-content: center;
				}
				`}
			</style>
		);

		const shadow = this.attachShadow({ mode: "open" });
		shadow.appendChild(style);

		this.#canvas = shadow.appendChild(
			<canvas width={0} height={0} css={{ maxWidth: "100%", height: "auto" }} />,
		) as HTMLCanvasElement;

		const offscreen = this.#canvas.transferControlToOffscreen();
		this.#worker.then((worker) => worker.canvas(Comlink.transfer(offscreen, offscreen ? [offscreen] : [])));

		// Set data- attributes and fire callbacks.
		this.#runConnectionStatus();
		this.#runRendererStatus();
	}

	public keydown(event: KeyboardEvent) {
		let signal = document.getElementById("signal");
		if (signal != null) signal.style.display = "block";
		console.log("keydown", event.key);
		this.#worker.then((worker) => worker.keydown(event.key));
	}
	public keyup(event: KeyboardEvent) {
		let signal = document.getElementById("signal");
		if (signal != null) signal.style.display = "none";
		this.#worker.then((worker) => worker.keyup(event.key));
	}
	public mousemove(event: MouseEvent) {
		this.#worker.then((worker) => worker.mousemove(event.pageX, event.pageY, document.getElementsByTagName("moq-watch-ui")[0].clientWidth, document.getElementsByTagName("moq-watch-ui")[0].clientHeight));
	}
	public mousedown(event: MouseEvent) {
		this.#worker.then((worker) => worker.mousedown(event.button));
	}
	public mouseup(event: MouseEvent) {
		this.#worker.then((worker) => worker.mouseup(event.button));
	}

	private urlChange(value: string) {
		// Get the PORT to use from the api endpoint.
		apiRequest("getPersonalStream").then(data => {
			let port: string = data.port;
			value = value.replace("${PORT}", port);

			console.log("Setting URL to:", value);
			this.#worker.then((worker) => worker.url(value));
		})
	}

	private pausedChange(value: boolean) {
		console.log("paused:", value);
		this.#worker.then((worker) => worker.paused(value));
	}

	private volumeChange(value: number) {
		if (value < 0 || value > 1) {
			throw new RangeError("volume must be between 0 and 1");
		}
		this.#worker.then((worker) => worker.volume(value));
	}

	// Set the target latency in ms.
	// A higher value means more stable playback.
	private latencyChange(ms: number) {
		if (ms < 0) {
			throw new RangeError("latency must be greater than 0");
		}

		this.#worker.then((worker) => worker.latency(ms));
	}

	async #runConnectionStatus() {
		for await (const state of this.connectionStatus()) {
			this.setAttribute("data-connection-status", state);
		}
	}

	async #runRendererStatus() {
		for await (const state of this.rendererStatus()) {
			this.setAttribute("data-renderer-status", state);
		}
	}

	async *connectionStatus(): AsyncGenerator<ConnectionStatus> {
		const worker = await this.#worker;
		const status = await Comlink.proxy(worker.status());

		for (;;) {
			const next = await status.connection();
			yield convertConnectionStatus(next);
		}
	}

	async *rendererStatus(): AsyncGenerator<RendererStatus> {
		const worker = await this.#worker;
		const status = await Comlink.proxy(worker.status());

		for (;;) {
			const next = await status.renderer();
			switch (next) {
				case Rust.RendererStatus.Idle:
					yield "idle";
					break;
				case Rust.RendererStatus.Paused:
					yield "paused";
					break;
				case Rust.RendererStatus.Buffering:
					yield "buffering";
					break;
				case Rust.RendererStatus.Live:
					yield "live";
					break;
				default: {
					const _exhaustive: never = next;
					throw new Error(_exhaustive);
				}
			}
		}
	}
}

declare global {
	interface HTMLElementTagNameMap {
		"moq-watch": Watch;
	}
}

class ConnectionStatusEvent extends CustomEvent<ConnectionStatus> {
	constructor(detail: ConnectionStatus) {
		super("moq-connection", { detail, bubbles: true, composed: true });
	}
}

class RendererStatusEvent extends CustomEvent<RendererStatus> {
	constructor(detail: RendererStatus) {
		super("moq-renderer", { detail, bubbles: true, composed: true });
	}
}

declare global {
	interface HTMLElementEventMap {
		"moq-connection": ConnectionStatusEvent;
		"moq-renderer": RendererStatusEvent;
	}
}

export default Watch;
