import express, {Request, Response} from "express";
import getPersonalStream from "./api.js";
import cors from "cors";

const app = express();
const port = 3000;

app.use(cors());

app.get("/getPersonalStream", (req: Request, res: Response) => {
    getPersonalStream(req, res);
});

app.listen(port, () => {
    console.log(`Fetch server is running at http://localhost:${port}`);
});