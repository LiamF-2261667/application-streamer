export async function apiRequest(action: string, method: "GET" | "POST" = "GET", data?: any): Promise<any> {
    await waitForApiToComeOnline();

    return rawApiRequest(action, method, data);
}

async function rawApiRequest(action: string, method: "GET" | "POST" = "GET", data?: any): Promise<any> {
    let res = await fetch("http://localhost:3000/" + action, {
        method,
        headers: {
            "Content-Type": "application/json"
        },
        body: method === "POST" ? JSON.stringify(data) : undefined
    });

    if (!res.ok) {
        throw new Error(`HTTP error! status: ${res.status}`);
    }
    return await res.json();
}

function waitForApiToComeOnline(): Promise<void> {
    return new Promise((resolve, reject) => {
        const interval = setInterval(async () => {
            try {
                console.log("Checking if API is online...");
                await rawApiRequest("isAlive");
                console.log("API is online!");
                clearInterval(interval);
                resolve();
            } catch (error) {
                // console.error("API not online yet, retrying...");
            }
        }, 5000);
    });
}