import express from "express";
import {getPersonalStream, heartbeat, isAlive} from "./api.js";
import cors from "cors";

const app = express();
const port = 3000;

app.use(cors());
app.use(express.json());

app.get("/getPersonalStream", getPersonalStream);
app.get("/isAlive", isAlive);
app.post("/heartbeat", heartbeat);

app.listen(port, () => {
    console.log(`Fetch server is running at http://localhost:${port}`);
});