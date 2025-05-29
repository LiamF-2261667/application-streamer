import {Container, DockerFile, Image, Tag} from "./docker.js";
import path from "node:path";

export default class Stream {
    private readonly _port: number;
    private _container?: Container;

    constructor(port: number) {
        this._port = port;
    }

    public get port(): number {
        return this._port;
    }

    public get container(): Container | undefined {
        return this._container;
    }

    public json(): object {
        return {
            port: this._port,
        };
    }

    public async start(): Promise<void> {
        if (!this._container) {
            this._container = await this.createContainer();
        }

        return this._container.start();
    }

    public stop(): Promise<void> {
        if (!this._container) {
            return Promise.resolve();
        }

        return this._container.stop();
    }

    private async createContainer(): Promise<Container> {
        let image = await (
            new DockerFile(
                path.join(__dirname, "../application-streamer/Dockerfile"),
                new Tag("application-streamer", "latest"),
                path.join(__dirname, "../application-streamer/")
            )
        ).build();

        return new Container(
            `stream-deliverer-${this._port}:latest`,
            image
        );
    }
}