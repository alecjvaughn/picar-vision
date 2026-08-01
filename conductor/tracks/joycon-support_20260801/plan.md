# Implementation Plan: Full JoyCon Support

## Phase 1: Input Mapping & Configuration Detection
- [ ] Task: Update the Rust (`gilrs`) backend to detect JoyCon connection states (Dual vs Single).
- [ ] Task: Map D-Pad, Face Buttons, and Triggers to respective payload commands in both configurations.
- [ ] Task: Test and verify inputs are correctly translated to the Python backend payload.

## Phase 2: Telemetry & UI Updates
- [ ] Task: Extract battery and detailed connection status (Left/Right) from the controller in Rust.
- [ ] Task: Update the Tauri event payload to send battery/connection telemetry to the frontend.
- [ ] Task: Redesign the "Controller Status" UI card in Svelte to display individual JoyCon states and battery levels.

## Phase 3: Calibration System
- [ ] Task: Implement calibration parameters (deadzones, center offsets) inside `ControlSettings` in Rust.
- [ ] Task: Apply calibration math to raw joystick inputs before calculating steering/throttle.
- [ ] Task: Create a Calibration Modal in the Svelte UI.
- [ ] Task: Add Tauri commands to capture calibration bounds and save them to Rust state.
