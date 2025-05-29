import Stream from "./stream.js";

class StreamManager {
    private streams: Stream[] = [];

    public newStream(): Stream {
        let free_ports = this.availablePorts();
        if (free_ports.length === 0) {
            throw new Error("No free ports available");
        }

        let port = free_ports[0];

        let stream = new Stream(port);
        this.streams.push(stream);

        stream.start().catch(error => {
            console.error(`Failed to start stream on port ${port}:`, error);
            this.streams = this.streams.filter(s => s.port !== port);
            throw new Error(`Failed to start stream on port ${port}: ${error.message}`);
        });

        console.log(`New stream created on port ${port}`);
        return stream;
    }

    private availablePorts(): number[] {
        let res: number[] = [];
        for (let i = 4443; i < 4493; i++) {
            res.push(i);
        }

        const usedPorts = this.streams.map(stream => stream.port);
        return res.filter(port => !usedPorts.includes(port));
    }
}

export const streamManager = new StreamManager();