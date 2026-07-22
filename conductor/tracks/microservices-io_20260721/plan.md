# Implementation Plan: Microservices Deployment & Pi I/O Testing

## Phase 1: Dockerize the Legacy Python Codebase
- [x] Task: Create Dockerfile for the Raspberry Pi environment
    - [x] Write a base `Dockerfile` targeting the Pi (e.g., `arm64v8/python:3.11-slim`)
    - [x] Install necessary system dependencies (e.g., OpenCV, I2C/GPIO tools)
    - [x] Create `requirements.txt` based on the legacy codebase needs
- [x] Task: Setup Docker Compose
    - [x] Create `docker-compose.yml` to orchestrate the backend services
    - [x] Mount necessary hardware devices (e.g., `/dev/i2c-1`, `/dev/video0`) to the container

## Phase 2: Hardware I/O Abstraction (Donkeycar compatibility)
- [x] Task: Abstract Motor & Servo Controls
    - [x] Create a `parts` package mimicking the Donkeycar parts structure
    - [x] Implement the PCA9685/L298N actuator class with `run()` method for steering/throttle
    - [x] Implement camera pan/tilt servo class
    - [x] Write Python unit tests for motor/servo inputs (mocking hardware)
- [x] Task: Abstract Sensor & Camera Ingestion
    - [x] Implement the OpenCV camera class to continuously capture frames
    - [x] Implement sensor classes (Ultrasonic, Photoresistor) to poll and yield data
    - [x] Write Python unit tests for sensor parsing

## Phase 3: WebSocket Gateway Implementation
- [x] Task: Setup WebSocket Server
    - [x] Integrate a Python WebSocket library (e.g., `websockets` or `fastapi` with `uvicorn`)
    - [x] Define JSON schemas for incoming commands (steering, throttle, pan, tilt)
    - [x] Define JSON schemas for outgoing telemetry (sensor data)
- [x] Task: Integrate I/O Parts into the Server Loop
    - [x] Create the main event loop that continuously reads camera frames/sensors
    - [x] Broadcast MJPEG frames and telemetry data to connected clients
    - [x] Route incoming commands to the motor/servo abstractions
- [x] Task: Client Testing
    - [x] Write a simple Python WebSocket client script to verify two-way communication and low latency (<100ms)

## Phase 4: Edge Deployment and Verification
- [x] Task: Deploy to Raspberry Pi
    - [x] Deploy the `docker-compose` stack to the Raspberry Pi
    - [x] Verify container health and logs
- [x] Task: Live I/O Testing
    - [x] Test live motor control via the test client
    - [x] Test live camera feed streaming via the test client
    - [x] Validate end-to-end latency is under 100ms

## Phase 5: User-Review (Automated Hardware Test Suite)
- [x] Task: Implement Automated Hardware Test Suite
    - [x] Create `hardware_test_suite.py` on the Mac client side
    - [x] Accept host IP as a command-line parameter
    - [x] Sequence 1: Test Motors (Forward, Backward, Left, Right)
    - [x] Sequence 2: Test Servos (Pan/Tilt sweeps)
    - [x] Sequence 3: Test Sensors (Poll and log ultrasonic and photoresistor)
    - [x] Sequence 4: Video Feed (Receive MJPEG frames and display them using cv2 on the Mac)
