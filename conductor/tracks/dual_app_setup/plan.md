# Dual App Setup Plan

- [x] Phase 1: Update Raspberry Pi Gateway
  - [x] Create a global set of connected WebSocket clients in `gateway.py`.
  - [x] Safely add/remove clients in the connection handler.
  - [x] Modify the command loop to intercept `{"type": "bounding_boxes"}` and broadcast it to all other connected clients.

- [x] Phase 2: Update Rust Backend (Desktop)
  - [x] In `websocket.rs`, intercept successful ONNX inference results in the `telemetry` handler.
  - [x] Serialize the bounding boxes into a `bounding_boxes` JSON message.
  - [x] Transmit the message through the `command_tx` WebSocket channel back to the Raspberry Pi.

- [x] Phase 3: Update Svelte Frontend (iOS)
  - [x] Svelte currently merges boxes from `telemetry` natively. Add an event listener for `json-message`.
  - [x] If `json-message` is received with `type: "bounding_boxes"`, update the `boundingBoxes` state to render the desktop's boxes.
  - [x] Update `aiDebugInfo` string to show "[Desktop Stream]" so the user knows where the boxes came from.
