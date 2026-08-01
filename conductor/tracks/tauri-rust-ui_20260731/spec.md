# Specification: Modern Desktop Interface (Tauri & Rust UI)

## Overview
Build the foundational desktop application for Picar-Vision using Tauri, Rust, and Svelte (with TypeScript). This app will serve as the primary command and control center, displaying the live video feed, telemetry data, and providing robust controls, specifically tailored for Nintendo Switch/Switch 2 JoyCons.

## Functional Requirements
- **Tauri/Svelte Application Setup:** Initialize a new Tauri project using the Svelte/TypeScript template.
- **Rust Backend Integration:**
  - Establish a WebSocket client in the Rust backend to connect to the Raspberry Pi gateway.
  - Rust backend processes incoming telemetry and MJPEG frames and emits them as Tauri events to the frontend.
- **Dashboard UI (Svelte):**
  - **Live Video Feed:** Display the MJPEG stream smoothly.
  - **Telemetry Panel:** Visualize distance, light sensors, and network latency.
  - **Connection Manager:** UI to input the Pi's IP address and connect/disconnect.
- **Controller Support (JoyCons):** Integrate Nintendo Switch / Switch 2 JoyCon support directly into the app for steering and throttle input, translating axes and button presses to WebSocket commands.

## Non-Functional Requirements
- **Low Latency:** UI updates and video rendering must remain smooth (< 100ms latency).
- **Architecture:** The Rust backend must remain decoupled from the UI, preparing for future YOLOv8 CV processing loops on the Rust side.

## Acceptance Criteria
- App successfully builds and launches on macOS.
- User can input the Pi's IP address and establish a connection.
- Video feed and telemetry data update in real-time in the UI.
- User can drive the car using connected Nintendo Switch / Switch 2 JoyCons.

## Out of Scope
- Support for Xbox, PlayStation, or generic controllers (planned for later).
- Integration of YOLOv8 computer vision (planned for a subsequent track).
- Infrastructure deployment (ArgoCD/Terraform) setup for the desktop app itself.
