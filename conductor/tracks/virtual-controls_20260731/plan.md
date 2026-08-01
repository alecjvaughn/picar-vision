# Implementation Plan: Virtual Controls & Sensitivity Dials

## Phase 1: UI Scaffolding & State Management
- [ ] Task: Scaffold Virtual Controls UI
    - [ ] Create UI components for Virtual Joystick, WASD keys, D-Pad, and Snap button in the Side Panel.
    - [ ] Add the Motor Speed and Servo Sensitivity range sliders.
- [ ] Task: Svelte State Wiring
    - [ ] Initialize reactive `$state` variables for the virtual keys, joystick position, and slider values.

## Phase 2: Keyboard & UI Event Mapping
- [ ] Task: Keyboard Event Listeners
    - [ ] Implement `keydown` and `keyup` listeners for physical WASD keys.
    - [ ] Bind these events to visually light up the virtual WASD keys and update the virtual joystick position.
    - [ ] Dispatch motor command events to the Rust backend based on WASD state, scaled by the Motor Speed dial.
- [ ] Task: Dial State Syncing
    - [ ] Sync the Motor Speed and Servo Sensitivity slider values with the Rust backend via Tauri IPC commands.

## Phase 3: Gamepad Servo Controls (Rust Backend)
- [ ] Task: Extend Controller Event Polling
    - [ ] Update `controller.rs` to map D-Pad inputs to servo pan/tilt logic.
    - [ ] Update `controller.rs` to map the Right Shoulder button to the snap-to-center logic.
- [ ] Task: Sensitivity and Command Generation
    - [ ] Track servo positions and apply the Servo Sensitivity dial value to calculate continuous movement.
    - [ ] Send new servo commands via the active WebSocket connection.
- [ ] Task: Emit Controller States to UI
    - [ ] Emit Tauri events when D-Pad or Right Shoulder buttons are pressed to light up the corresponding virtual keys in the Svelte UI.

## Phase 4: Integration & Validation
- [ ] Task: WebSocket Payload Updates
    - [ ] Ensure the JSON command payloads generated in Rust correctly bundle both motor (throttle/steering) and servo (pan/tilt) values.
- [ ] Task: Code Quality & Linting
    - [ ] Run `cargo clippy` and `cargo fmt` on the Rust backend.
    - [ ] Run `npm run check` and formatting on the Svelte UI.
    - [ ] Verify low latency and visual responsiveness of all virtual keys.
