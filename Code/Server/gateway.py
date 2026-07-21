import asyncio
import websockets
import json
import base64
import cv2
import time
from parts.actuators import FreenoveMotorPart, FreenoveServoPart
from parts.sensors import UltrasonicPart, PhotoresistorPart
from parts.camera import OpenCVCameraPart

# Initialize parts
try:
    motor = FreenoveMotorPart()
    servo = FreenoveServoPart()
    ultrasonic = UltrasonicPart()
    photo = PhotoresistorPart()
    camera = OpenCVCameraPart(width=320, height=240, framerate=15) # lower res for network speed
except Exception as e:
    print(f"Error initializing parts (hardware may not be present): {e}")

async def telemetry_loop(websocket):
    """Continuously send telemetry (sensor data & camera frames) to client."""
    while True:
        try:
            # Gather sensors
            distance = ultrasonic.run()
            left_light, right_light = photo.run()
            
            # Gather frame and encode to JPEG
            frame = camera.run()
            frame_b64 = ""
            if frame is not None:
                ret, buffer = cv2.imencode('.jpg', frame, [int(cv2.IMWRITE_JPEG_QUALITY), 60])
                if ret:
                    frame_b64 = base64.b64encode(buffer).decode('utf-8')

            telemetry_data = {
                "type": "telemetry",
                "distance": distance,
                "left_light": left_light,
                "right_light": right_light,
                "frame": frame_b64,
                "timestamp": time.time()
            }
            
            await websocket.send(json.dumps(telemetry_data))
            await asyncio.sleep(0.05) # 20 Hz telemetry
        except websockets.exceptions.ConnectionClosed:
            break
        except Exception as e:
            print(f"Telemetry error: {e}")
            break

async def command_loop(websocket):
    """Receive commands from the client and route to actuators."""
    async for message in websocket:
        try:
            data = json.loads(message)
            if data.get("type") == "command":
                steering = data.get("steering", 0.0)
                throttle = data.get("throttle", 0.0)
                pan = data.get("pan", 0.0)
                tilt = data.get("tilt", 0.0)
                
                motor.run(steering, throttle)
                servo.run(pan, tilt)
        except Exception as e:
            print(f"Command error: {e}")

async def handler(websocket):
    print(f"Client connected: {websocket.remote_address}")
    
    # Run both loops concurrently
    telemetry_task = asyncio.create_task(telemetry_loop(websocket))
    command_task = asyncio.create_task(command_loop(websocket))
    
    done, pending = await asyncio.wait(
        [telemetry_task, command_task],
        return_when=asyncio.FIRST_COMPLETED,
    )
    for task in pending:
        task.cancel()
        
    print(f"Client disconnected: {websocket.remote_address}")

async def main():
    print("Starting WebSocket gateway on ws://0.0.0.0:8765")
    async with websockets.serve(handler, "0.0.0.0", 8765):
        await asyncio.Future()  # run forever

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass
    finally:
        motor.shutdown()
        servo.shutdown()
        ultrasonic.shutdown()
        photo.shutdown()
        camera.shutdown()
