import asyncio
import websockets
import json
import time

async def listen(websocket):
    while True:
        try:
            message = await websocket.recv()
            data = json.loads(message)
            if data.get("type") == "telemetry":
                server_ts = data.get("timestamp")
                if server_ts:
                    latency = (time.time() - server_ts) * 1000
                    print(f"Latency: {latency:.2f} ms | Distance: {data.get('distance')}cm | Frame Size: {len(data.get('frame', ''))} bytes")
        except Exception as e:
            print(f"Listen error: {e}")
            break

async def send_commands(websocket):
    while True:
        try:
            command = {
                "type": "command",
                "steering": 0.5,
                "throttle": 0.5,
                "pan": 0.0,
                "tilt": 0.0
            }
            await websocket.send(json.dumps(command))
            await asyncio.sleep(0.1) # 10 Hz commands
        except Exception as e:
            print(f"Send error: {e}")
            break

async def main():
    uri = "ws://localhost:8765"
    async with websockets.connect(uri) as websocket:
        print(f"Connected to WebSocket gateway at {uri}")
        listen_task = asyncio.create_task(listen(websocket))
        command_task = asyncio.create_task(send_commands(websocket))
        
        await asyncio.wait([listen_task, command_task], return_when=asyncio.FIRST_COMPLETED)

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass
