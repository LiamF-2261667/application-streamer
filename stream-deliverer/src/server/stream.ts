export default class Stream {
    private readonly _port: number;

    constructor(port: number) {
        this._port = port;
    }

    public get port(): number {
        return this._port;
    }

    public json(): object {
        return {
            port: this._port,
        };
    }
}