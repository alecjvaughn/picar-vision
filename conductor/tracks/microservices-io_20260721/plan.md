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
- [ ] Task: Abstract Motor & Servo Controls
    - [ ] Create a `parts` package mimicking the Donkeycar parts structure
    - [ ] Implement the PCA9685/L298N actuator class with `run()` method for steering/throttle
    - [ ] Implement camera pan/tilt servo class
    - [ ] Write Python unit tests for motor/servo inputs (mocking hardware)
- [ ] Task: Abstract Sensor & Camera Ingestion
    - [ ] Implement the OpenCV camera class to continuously capture frames
    - [ ] Implement sensor classes (Ultrasonic, Photoresistor) to poll and yield data
    - [ ] Write Python unit tests for sensor parsing

## Phase 3: WebSocket Gateway Implementation
- [ ] Task: Setup WebSocket Server
    - [ ] Integrate a Python WebSocket library (e.g., `websockets` or `fastapi` with `uvicorn`)
    - [ ] Define JSON schemas for incoming commands (steering, throttle, pan, tilt)
    - [ ] Define JSON schemas for outgoing telemetry (sensor data)
- [ ] Task: Integrate I/O Parts into the Server Loop
    - [ ] Create the main event loop that continuously reads camera frames/sensors
    - [ ] Broadcast MJPEG frames and telemetry data to connected clients
    - [ ] Route incoming commands to the motor/servo abstractions
- [ ] Task: Client Testing
    - [ ] Write a simple Python WebSocket client script to verify two-way communication and low latency (<100ms)

## Phase 4: Edge Deployment and Verification
- [ ] Task: Deploy to Raspberry Pi
    - [ ] Deploy the `docker-compose` stack to the Raspberry Pi
    - [ ] Verify container health and logs
- [ ] Task: Live I/O Testing
    - [ ] Test live motor control via the test client
    - [ ] Test live camera feed streaming via the test client
    - [ ] Validate end-to-end latency is under 100ms
