# Picar-Vision Architecture

## Overview
This document tracks the architectural decisions, diagrams, requirements, and problem solutions for the Picar-Vision project.

## Requirements
- Replace the legacy raw TCP socket architecture with a robust, low-latency WebSocket gateway.
- Ensure the Raspberry Pi edge deployment is fully containerized using Docker, avoiding manual dependency installations.
- Abstract the hardware I/O (motors, servos, sensors, camera) into a "Parts" architecture, enabling future plug-and-play compatibility with the Donkeycar AI framework.
- Provide automated hardware testing from the desktop client to verify the telemetry and command streams.

## Solutions & Architectural Changes

### Changes to Original Source Code & Folders
To transition to the microservices and Donkeycar-compatible architecture, the following changes were made to the original codebase:

1. **New `Code/Server/parts/` Abstraction Layer:**
   - Created `parts/actuators.py`: Wraps `motor.py` and `servo.py` into `FreenoveMotorPart` and `FreenoveServoPart`.
   - Created `parts/sensors.py`: Wraps `ultrasonic.py` and `photoresistor.py` into `UltrasonicPart` and `PhotoresistorPart`.
   - Created `parts/camera.py`: Wraps `camera.py` into `OpenCVCameraPart`.
   - *Rationale:* This decouples the low-level hardware drivers from the network server, allowing any AI or controller to independently poll and command these "parts."

2. **WebSocket Gateway Implementation:**
   - Created `Code/Server/gateway.py`: Replaces the legacy TCP server with an `asyncio` WebSocket server.
   - It runs two concurrent loops: a `telemetry_loop` broadcasting sensor data/MJPEG frames at 20Hz, and a `command_loop` receiving steering/throttle inputs.

3. **Dockerization (Edge Deployment):**
   - Created `Code/Server/Dockerfile` and `docker-compose.yml` (in the project root).
   - Optimized build times by transitioning from `python:3.11-slim` to `debian:bookworm-slim`, allowing heavy dependencies (`python3-opencv`, `python3-numpy`) to be installed via `apt-get` rather than compiled from source via `pip`.

4. **Testing & Client Scripts:**
   - Added `Code/Server/test_client.py`: A lightweight client for quick I/O validation.
   - Added `Code/Server/hardware_test_suite.py`: An automated testing sequence that cycles through motor movements, servo sweeps, and displays the OpenCV video stream natively on macOS.

## System Diagram
```mermaid
graph TD
    Client[Mac Desktop Client] -->|WebSocket: Commands| Gateway[gateway.py]
    Gateway -->|WebSocket: Telemetry/Video| Client
    
    subgraph Raspberry Pi (Docker Container)
        Gateway -->|Steering/Throttle| Actuators[parts/actuators.py]
        Gateway -->|Poll| Sensors[parts/sensors.py]
        Gateway -->|Read Frame| Camera[parts/camera.py]
        
        Actuators --> HW_I2C[I2C / GPIO]
        Sensors --> HW_I2C
        Camera --> HW_CSI[CSI Camera]
    end
```
