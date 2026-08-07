# Implementation Plan: Advanced Tracking Modes

## Phase 1: UI Updates & Object Selection
- [x] Task: Move the "Enable AI" toggle and Target Class selector into the main window, merging it with the bottom panel.
- [x] Task: Add a way to easily choose what to lock onto (Person vs Hand) and clear the current tracking command.
- [x] Task: Implement a dead-man switch on the virtual controller (so AI/movement stops if let go).

## Phase 2: Lock Mode (Camera Pan/Tilt) Fixes & Implementation
- [x] Task: Fix the inverted camera pan/tilt logic in `websocket.rs` causing the camera to force itself into the extreme upper left position due to positive feedback.
- [x] Task: Allow tracking moving Person OR Hand using the CoreML inferences.
- [x] Task: Apply hardcoded servo limits and deadzones to the calculated pan/tilt angles.
- [x] Task: Test and verify the camera can track a moving person smoothly while the car is stationary.

## Phase 3: Follow Mode (Car Steering/Throttle) Implementation
- [ ] Task: Implement a gamepad dead-man switch requirement (e.g. holding Right Trigger) before autonomous motor commands are sent.
- [ ] Task: Calculate throttle based on the bounding box height/area (distance estimation) and steering based on the pan angle or x-offset.
- [ ] Task: Add hardcoded speed limits to prevent runaway behavior.
- [ ] Task: Implement the 1-second timeout loss-of-signal safety stop.

## Phase 4: Final Testing & Documentation
- [ ] Task: Perform end-to-end testing of Lock and Follow modes.
- [ ] Task: Update `conductor/in-flight-changes.md` to document the new features.
- [ ] Task: Commit changes via `git` with conventional commits on a new branch.
