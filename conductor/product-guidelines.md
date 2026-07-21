# Product Guidelines: Picar-Vision

## 1. Design & UI/UX Principles
- **Modern & Responsive:** Utilize a sleek, dark-themed UI suited for telemetry and video feeds. The interface should feel native, snappy, and hardware-accelerated (via Tauri).
- **HUD-First Dashboard:** The primary interface should prioritize the real-time YOLOv8 video feed and essential telemetry (battery, speed, connection status) over configuration menus.
- **Micro-Animations:** Use subtle animations to indicate connection state changes and incoming data streams without causing visual fatigue.
- **Error Transparency:** Clearly indicate when connection to the Raspberry Pi is lost or when video frames are dropping, providing actionable recovery steps.

## 2. Technical Guidelines
- **Performance First:** Since real-time video and object detection (YOLOv8) are resource-intensive, the Rust backend must handle heavy lifting, maintaining low memory footprints and avoiding blocking the UI thread.
- **Asynchronous Communication:** Use WebSockets or asynchronous TCP streams for bi-directional communication between the desktop app and the Raspberry Pi to ensure minimal latency.
- **Cross-Platform Compatibility:** While tailored for desktop, ensure the Tauri application can build natively on macOS, Windows, and Linux without platform-specific hacks.
- **Modular Hardware Abstraction (Donkeycar Compatibility):** The underlying interfaces communicating with the car's sensors (camera) and actuators (motors) must be heavily abstracted. This ensures that a Donkeycar "Part" can easily ingest our telemetry data and emit control overrides, acting as a drop-in autopilot component down the line.

## 3. Deployment & DevOps (GitOps)
- **Infrastructure as Code:** All supporting infrastructure (e.g., edge configuration, simulated environments) must be defined using Terraform and deployed via ArgoCD onto a local `kind` cluster.
- **CI/CD Reliability:** GitHub Actions should validate Rust formatting (clippy), run tests, and build the Tauri binaries for releases automatically.
