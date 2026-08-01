# In-Flight Changes Log

This document tracks changes requested by the user and implemented during active development tracks. It serves to logically group bugs, features, and fixes for easier review and changelog generation.

## Track: `virtual-controls_20260731`

### 1. Control & Input Mechanics (Bugs & Enhancements)
- **Multi-Touch Support:** Re-engineered virtual joystick event listeners to track individual `pointerId`s, enabling simultaneous driving and camera panning. 
  - Commits: `c0af494`
- **Virtual Key Responsiveness:** Upgraded virtual arrow and D-Pad buttons from `mousedown` to responsive `pointerdown`/`pointerup`/`pointerleave` events.
  - Commits: `aaa762d`, `3a5ac9e`
- **Gamepad State Conflict:** Fixed physical gamepad loop constantly overriding the web UI's camera state when only driving.
  - Commits: `aaa762d`

### 2. Camera & Steering Logic (Features)
- **Viewport Turn (Drive Where You Look):** Implemented a driving mode where vehicle steering mirrors the camera's horizontal pan angle.
  - Commits: `283105d`, `ccd3bfc`
- **Camera Modes (Absolute vs Incremental):** Added absolute/incremental paradigms. Scaled the polling loops to 60Hz to smooth out Incremental mode, but ultimately disabled it entirely due to hardware/servo spasming issues, forcing Absolute mode as the stable default.
  - Commits: `460a789`, `45eb22a`, `0343d33`, `51a0299`
- **Sensitivity & Scaling:** Enforced numeric type-casting on `servoSensitivity` and `motorSpeed` sliders and bound them to real-time `oninput` handlers.
  - Commits: `3a5ac9e`

### 3. UI / UX Organization (Enhancements)
- **Camera Modes UI Grouping:** Logically split and stacked camera options into "Camera Mode" (Absolute/Incremental) and "Turning Style" (Normal/Follow Camera).
  - Commits: `ccd3bfc`, `59b0a21`

## Track: `joycon-support_20260801`

### 1. Gamepad & Hardware (Features)
- **JoyCon Connection & Telemetry:** Updated `gilrs` loop in Rust to emit power information (battery percentage) and identify JoyCon types (Dual, Single L, Single R).
  - Commits: `e19a0a1`
- **Joystick Calibration:** Created an interactive modal in Svelte and backing Rust state to capture, persist, and apply individual stick deadzones and center offsets to prevent drift.
  - Commits: `e19a0a1`
