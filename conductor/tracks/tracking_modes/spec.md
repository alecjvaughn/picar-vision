# Specification: Advanced Tracking Modes

## Overview
Implement advanced tracking modes in PiCar Vision that allow the camera to lock onto a target and the car to physically follow the target. Due to current model constraints, "person" will be used as a proxy for the requested "hand" tracking.

## Functional Requirements
1. **Target Selection:** The system will use the YOLOv8 "person" class to identify the target for tracking.
2. **Lock Mode (Camera Only):** 
   - When engaged, the pan and tilt servos will automatically adjust to keep the detected target in the center of the camera frame.
   - The car's drive motors remain stationary.
3. **Follow Mode (Camera + Drive):**
   - Incorporates Lock Mode behavior (camera remains locked on target).
   - The car will steer and drive forward/backward to maintain a set distance from the target.
   - Requires a dead-man switch on the gamepad to be actively held to permit any autonomous movement.

## Non-Functional & Safety Requirements
1. **Speed Limits:** A hard-coded safety limit for the drive motor speed during Follow mode to prevent runaway behavior.
2. **Servo Limits:** Maximum turn angles for the pan/tilt servos must be enforced to prevent mechanical binding or over-extension.
3. **Loss of Signal:** If the object is lost from the vision frame for more than 1 second, the car must automatically stop driving and wait for re-acquisition or manual intervention.

## Out of Scope
- Training or integrating a specialized hand-detection model (deferred to a future track).
