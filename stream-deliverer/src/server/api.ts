import {Request, Response} from "express";
import {streamManager} from "./streamManager.js";

export async function getPersonalStream(req: Request, res: Response) {
    try {
        let stream = await streamManager.newStream();

        res.status(200).json(stream.json());
    }
    catch (error) {
        console.error("Error creating personal stream:", error);
        res.status(500).json({ error });
    }
}

export function isAlive(req: Request, res: Response) {
    res.status(200).json({ alive: true });
}

export function heartbeat(req: Request, res: Response) {
    const port = req.body.port || req.query.port;
    if (isNaN(port)) {
        return res.status(400).json({ error: "Invalid port number" });
    }

    const stream = streamManager.getStream(port);
    if (!stream) {
        return res.status(404).json({ error: "Stream not found" });
    }

    stream.heartbeat();
    res.status(200).json({ message: "Heartbeat received" });
}