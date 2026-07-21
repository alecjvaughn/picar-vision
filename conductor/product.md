# Initial Concept
Build a desktop app for a vision-enabled raspi3b+ freenove 4wd base. Start with the base code in @[/Users/alecjvaughn/Developer/freenove-picar]. Use Tauri/Rust, kind/Terraform/ArgoCD/GHA, and YOLOv8. Add future compatibility for the Donkeycar framework, incorporating its principles into the design.

# Product Guide: Picar-Vision

## Vision
To build a high-performance, modern desktop application that serves as the command, control, and vision processing center for a Raspberry Pi 3B+ powered Freenove 4WD smart car. The underlying architecture must be designed to eventually serve as a plug-and-play front-end and telemetry layer for the **Donkeycar framework**, bridging the gap between manual remote operation and future AI-driven autonomous behavioral cloning.

## Target Audience
- Robotics enthusiasts and developers.
- Edge AI researchers and hobbyists working with computer vision.
- Users who need a reliable interface to remote-control and monitor the smart car.

## Core Features
- **Remote Telemetry and Control:** Low-latency communication to steer the car, control servos, and monitor sensor data.
- **Advanced Computer Vision:** Integration of YOLOv8 for real-time object detection and tracking, replacing the legacy Haar Cascade implementation.
- **Modern Desktop Interface:** A fast, responsive, and resource-efficient desktop UI built with Tauri and Rust.
- **Robust Infrastructure:** Automated, GitOps-driven deployment using GitHub Actions, Terraform, kind (Kubernetes IN Docker), and ArgoCD for reliable software delivery and potential cloud-edge integrations.
- **Donkeycar-Ready Architecture:** Clean separation of inputs (joystick/keyboard), state (telemetry/video), and outputs (motor commands) so that Donkeycar's "parts" concept can easily slot into the control loop in the future.

## Non-Goals
- Full autonomous driving without user supervision (for the initial MVP, though the architecture will support Donkeycar AI integration later).
- Support for hardware bases other than the Freenove 4WD kit.
- Mobile application counterparts (focus is strictly on the desktop experience).
