import asyncio
import websockets
import json
import base64
import cv2
import numpy as np
import time
import sys

async def display_video_and_telemetry(websocket):
    """Listens for telemetry and displays the video stream and sensor data."""
    try:
        while True:
            message = await websocket.recv()
            data = json.loads(message)
            if data.get("type") == "telemetry":
                # Print telemetry
                dist = data.get("distance")
                ll = data.get("left_light")
                rl = data.get("right_light")
                print(f"Sensors -> Distance: {dist} cm | Light: L={ll}V R={rl}V", end='\r')

                # Display video
                frame_b64 = data.get("frame")
                if frame_b64:
                    frame_bytes = base64.b64decode(frame_b64)
                    np_arr = np.frombuffer(frame_bytes, np.uint8)
                    frame = cv2.imdecode(np_arr, cv2.IMREAD_COLOR)
                    if frame is not None:
                        cv2.imshow("Picar-Vision Camera Stream", frame)
                        if cv2.waitKey(1) & 0xFF == ord('q'):
                            break
    except Exception as e:
        print(f"\nTelemetry stopped: {e}")
    finally:
        cv2.destroyAllWindows()

async def send_command(websocket, steering, throttle, pan, tilt, duration, name):
    print(f"\n[Testing] {name} (duration: {duration}s)")
    command = {
        "type": "command",
        "steering": steering,
        "throttle": throttle,
        "pan": pan,
        "tilt": tilt
    }
    
    start = time.time()
    while time.time() - start < duration:
        await websocket.send(json.dumps(command))
        await asyncio.sleep(0.1)

async def run_hardware_sequence(websocket, component=None):
    """Runs through a defined test sequence for the motors and servos."""
    print("\n--- Starting Hardware Test Sequence ---")
    await asyncio.sleep(2) # Wait for video to initialize

    if component in [None, "motor"]:
        # Motor Tests
        await send_command(websocket, 0.0, 0.5, 0.0, 0.0, 2.0, "Forward")
        await send_command(websocket, 0.0, -0.5, 0.0, 0.0, 2.0, "Backward")
        await send_command(websocket, 0.5, 0.0, 0.0, 0.0, 2.0, "Turn Right")
        await send_command(websocket, -0.5, 0.0, 0.0, 0.0, 2.0, "Turn Left")
        await send_command(websocket, 0.0, 0.0, 0.0, 0.0, 1.0, "Stop Motors")

    if component in [None, "servo"]:
        # Servo Tests
        await send_command(websocket, 0.0, 0.0, 0.5, 0.0, 1.5, "Pan Right")
        await send_command(websocket, 0.0, 0.0, -0.5, 0.0, 1.5, "Pan Left")
        await send_command(websocket, 0.0, 0.0, 0.0, 0.5, 1.5, "Tilt Up")
        await send_command(websocket, 0.0, 0.0, 0.0, -0.5, 1.5, "Tilt Down")
        await send_command(websocket, 0.0, 0.0, 0.0, 0.0, 1.0, "Center Servos")

    if component in ["camera", "sensors"]:
        print("Testing camera/sensors only. Keep window open to view telemetry/video. Press 'q' or Ctrl+C to exit.")
        while True:
            await asyncio.sleep(1)
            
    print("\n--- Hardware Test Sequence Complete ---")
    print("Keep the window open to view telemetry/video. Press 'q' in the video window or Ctrl+C to exit.")
    
    # Wait indefinitely so the telemetry task can keep running
    while True:
        await asyncio.sleep(1)

async def main():
    import argparse
    parser = argparse.ArgumentParser(description="Picar-Vision Hardware Test Suite")
    parser.add_argument("host", help="IP address or hostname of the Raspberry Pi")
    parser.add_argument(
        "--component", 
        choices=["all", "motor", "servo", "camera", "sensors"], 
        default="all",
        help="Specify which component to test (default: all)"
    )
    args = parser.parse_args()

    host = args.host
    component = None if args.component == "all" else args.component
    uri = f"ws://{host}:8765"
    
    print(f"Connecting to Picar-Vision Gateway at {uri}...")
    try:
        async with websockets.connect(uri) as websocket:
            print("Connected! Initializing test suite...")
            
            # Start telemetry/video loop and command sequence concurrently
            display_task = asyncio.create_task(display_video_and_telemetry(websocket))
            sequence_task = asyncio.create_task(run_hardware_sequence(websocket, component))
            
            await asyncio.wait([display_task, sequence_task], return_when=asyncio.FIRST_EXCEPTION)
    except Exception as e:
        print(f"Connection failed: {e}")

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nExiting Test Suite.")
