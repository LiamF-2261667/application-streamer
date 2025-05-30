import {Container, DockerFile, Image, PortMapping, Tag} from "./docker.js";
import path from "node:path";

const DEFAULT_IMAGE: Image = await (async () => {
    let file_location = path.resolve(process.cwd(), "application-streamer/Dockerfile").replace(/\\/g, "/");
    let context_location = file_location.replace(/\/Dockerfile$/, "");

    console.log("------------------------------------------------------------");
    console.log(`Building Docker image for stream deliverer...`);
    console.log(`Using Dockerfile at: ${file_location}`);
    console.log(`Using context at: ${context_location}`);
    console.log("------------------------------------------------------------");

    return await (
        new DockerFile(
            file_location,
            new Tag("application-streamer", "latest"),
            context_location
        )
    ).build();
})();
const APPLICATION_PORT: number = 4443;
const KEEP_ALIVE_TIMER: number = 1000 * 60; // 1 minute

export default class Stream {
    private readonly _port: number;
    private readonly _container: Container;
    private self_destruct: NodeJS.Timeout | null = null;

    constructor(port: number) {
        this._port = port;

        let container = new Container(
            `application-streamer-${this._port}`,
            DEFAULT_IMAGE
        );
        container.addPort(new PortMapping(APPLICATION_PORT, port));

        this._container = container;
    }

    public get port(): number {
        return this._port;
    }

    public get container(): Container {
        return this._container;
    }

    public json(): object {
        return {
            port: this._port,
        };
    }

    public async start(): Promise<void> {
        console.log(`Starting stream on port ${this._port}...`);
        let _ = this._container.start();

        // Wait for the container to start and the stdout to be available
        while (!this._container.stdout) {
            await new Promise(resolve => setTimeout(resolve, 1000));
        }

        return new Promise((resolve) => {
            this._container.stdout.on('data', (data: any) => {
                if (data.toString().includes("Ready to accept connections")) {
                    console.log(`Stream on port ${this._port} is ready.`);

                    this.setSelfDestructTimer();
                    resolve();
                }
                else {
                    console.log(`Stream on port ${this._port} output:`, data.toString().replace(/\n$/, ""));
                }
            });
        });
    }

    public stop(): Promise<void> {
        console.log(`Stopping stream on port ${this._port}...`);
        return this._container.stop();
    }

    /**
     * Heartbeat method to keep the stream alive.
     * This method should be called periodically to prevent the stream from self-destructing.
     * If the stream is idle for too long, it will automatically stop.
     */
    public heartbeat(): void {
        if (this.self_destruct) {
            this.setSelfDestructTimer();
        }
    }

    private setSelfDestructTimer(): void {
        if (this.self_destruct) {
            clearTimeout(this.self_destruct);
        }

        this.self_destruct = setTimeout(() => {
            console.log(`Stream on port ${this._port} has been idle for too long. Stopping...`);
            this.stop().catch(err => console.error(`Failed to stop stream on port ${this._port}:`, err));
        }, KEEP_ALIVE_TIMER);
    }
}