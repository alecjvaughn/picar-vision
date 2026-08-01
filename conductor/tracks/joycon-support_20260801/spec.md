# Specification: Full JoyCon Support

## Overview
Implement comprehensive Nintendo JoyCon support for Picar-Vision, enhancing the existing gamepad capabilities. This includes supporting various physical configurations, exposing battery telemetry to the UI, and providing an in-app calibration tool for optimal control accuracy.

## Functional Requirements
- **Configuration Support:** Automatically detect and support both Dual JoyCons (used together as a single gamepad) and Single JoyCon (used sideways) configurations.
- **Input Mapping:** Fully map all supported physical inputs (L/R Joysticks, D-Pad, Face Buttons, and L/R/ZL/ZR Triggers) to vehicle controls (Steering, Throttle, Camera Pan/Tilt).
- **UI Telemetry:** Enhance the Svelte UI to display the connection status and battery levels for each connected JoyCon.
- **In-App Calibration:** Add a calibration modal/screen within the Tauri app to allow users to calibrate joystick deadzones and center points.

## Non-Functional Requirements
- Input handling must remain low-latency within the Rust controller loop.
- Calibration settings should persist across sessions.

## Acceptance Criteria
- User can drive and control the camera seamlessly using either a pair of JoyCons or a single JoyCon.
- The UI accurately updates the battery percentage and connection state for left and right JoyCons.
- Users can access the calibration modal, perform a calibration sequence, and the new deadzones/center points are actively applied to the control loop.

## Out of Scope
- Gyro/Motion controls.
- Rumble/Haptic feedback.
