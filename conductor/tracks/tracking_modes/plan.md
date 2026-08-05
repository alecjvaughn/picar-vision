# Implementation Plan: Advanced Tracking Modes

## Phase 1: State & Telemetry Updates
- [ ] Task: Add bounding box center calculation and telemetry transmission to `gateway.py`.
- [ ] Task: Expose tracking mode configuration variables (Lock vs Follow modes) in `VisionState` (Rust).
- [ ] Task: Ensure UI toggles or commands are sent over websocket to activate Lock or Follow.

## Phase 2: Lock Mode (Camera Pan/Tilt) Implementation
- [ ] Task: Add PID-like logic in Rust (or within `gateway.py` depending on architecture) to calculate pan/tilt corrections based on the target bounding box offset from the frame center.
- [ ] Task: Apply hardcoded servo limits and deadzones to the calculated pan/tilt angles.
- [ ] Task: Test and verify the camera can track a moving person smoothly while the car is stationary.

## Phase 3: Follow Mode (Car Steering/Throttle) Implementation
- [ ] Task: Implement a gamepad dead-man switch requirement (e.g. holding Right Trigger) before autonomous motor commands are sent.
- [ ] Task: Calculate throttle based on the bounding box height/area (distance estimation) and steering based on the pan angle or x-offset.
- [ ] Task: Add hardcoded speed limits to prevent runaway behavior.
- [ ] Task: Implement the 1-second timeout loss-of-signal safety stop.

## Phase 4: Final Testing & Documentation
- [ ] Task: Perform end-to-end testing of Lock and Follow modes.
- [ ] Task: Update `conductor/in-flight-changes.md` to document the new features.
- [ ] Task: Commit changes via `git` with conventional commits on a new branch.
