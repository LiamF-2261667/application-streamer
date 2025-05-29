import {Request, Response} from "express";
import {streamManager} from "./streamManager.js";

export default function getPersonalStream(req: Request, res: Response) {
    try {
        let stream = streamManager.newStream();

        res.status(200).json(stream.json());
    }
    catch (error) {
        console.error("Error creating personal stream:", error);
        res.status(500).json({ error });
    }
}