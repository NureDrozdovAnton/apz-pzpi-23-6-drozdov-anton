// Інтерфейс події зміни стану об'єкта
interface CanvasOperation {
    fileId: string
    userId: string
    type: "MOVE" | "RESIZE" | "COLOR"
    objectId: string
    payload: { x: number; y: number }
}

class MultiplayerClient {
    private ws: WebSocket

    constructor(fileId: string) {
        // Встановлення постійного з'єднання з Multiplayer сервером
        this.ws = new WebSocket(
            `wss://figma-multiplayer.example.com/files/${fileId}`,
        )
        this.ws.binaryType = "arraybuffer"

        this.ws.onmessage = (event) => {
            this.handleIncomingOperation(event.data)
        }
    }

    // Викликається рушієм рендерингу при перетягуванні мишкою
    public sendMoveOperation(objectId: string, newX: number, newY: number) {
        const op: CanvasOperation = {
            fileId: "file_12345",
            userId: "user_987",
            type: "MOVE",
            objectId: objectId,
            payload: { x: newX, y: newY },
        }

        const message = JSON.stringify(op)
        this.ws.send(message)
    }

    private handleIncomingOperation(data: string) {
        const op: CanvasOperation = JSON.parse(data)
        window.wasmEngine.updateObjectPosition(
            op.objectId,
            op.payload.x,
            op.payload.y,
        )
    }
}
