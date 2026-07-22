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

async def send_command(websocket, steering=0.0, throttle=0.0, pan=0.0, tilt=0.0, buzzer=False, led="off", duration=1.0, name="Command"):
    print(f"\n[Testing] {name} (duration: {duration}s)")
    command = {
        "type": "command",
        "steering": steering,
        "throttle": throttle,
        "pan": pan,
        "tilt": tilt,
        "buzzer": buzzer,
        "led": led
    }
    
    start = time.time()
    while time.time() - start < duration:
        await websocket.send(json.dumps(command))
        await asyncio.sleep(0.1)

async def run_hardware_sequence(websocket, component=None):
    """Runs through a defined test sequence for the components."""
    print("\n--- Starting Hardware Test Sequence ---")
    await asyncio.sleep(2) # Wait for video to initialize

    if component in [None, "motor"]:
        # Motor Tests
        await send_command(websocket, throttle=0.5, duration=2.0, name="Forward")
        await send_command(websocket, throttle=-0.5, duration=2.0, name="Backward")
        await send_command(websocket, steering=0.5, duration=2.0, name="Turn Right")
        await send_command(websocket, steering=-0.5, duration=2.0, name="Turn Left")
        await send_command(websocket, duration=1.0, name="Stop Motors")

    if component in [None, "servo"]:
        # Servo Tests
        await send_command(websocket, pan=0.5, duration=1.5, name="Pan Right")
        await send_command(websocket, pan=-0.5, duration=1.5, name="Pan Left")
        await send_command(websocket, tilt=0.5, duration=1.5, name="Tilt Up")
        await send_command(websocket, tilt=-0.5, duration=1.5, name="Tilt Down")
        await send_command(websocket, duration=1.0, name="Center Servos")

    if component in [None, "buzzer"]:
        # Buzzer Tests
        await send_command(websocket, buzzer=True, duration=0.5, name="Buzzer ON")
        await send_command(websocket, buzzer=False, duration=0.5, name="Buzzer OFF")
        await send_command(websocket, buzzer=True, duration=0.5, name="Buzzer ON")
        await send_command(websocket, buzzer=False, duration=0.5, name="Buzzer OFF")

    if component in [None, "led"]:
        # LED Tests
        await send_command(websocket, led="blink", duration=3.0, name="LED Blink")
        await send_command(websocket, led="rainbow", duration=3.0, name="LED Rainbow")
        await send_command(websocket, led="off", duration=1.0, name="LED OFF")

    if component in ["camera", "sensors", "photoresistor", "ultrasonic"]:
        print(f"Testing {component} only. Keep window open to view telemetry/video. Press 'q' or Ctrl+C to exit.")
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
        "-c", "--component", 
        choices=["all", "motor", "servo", "camera", "sensors", "buzzer", "led", "photoresistor", "ultrasonic"], 
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
