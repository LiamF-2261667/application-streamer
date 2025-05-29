export async function apiRequest(action: string, method: "GET" | "POST" = "GET", data?: any): Promise<any> {
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