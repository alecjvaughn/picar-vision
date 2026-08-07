## PiCar-Vision

> **Note:** This repository is a heavily customized fork of a generic 4WD smart car kit, modified to include a modern microservices architecture, a Rust-based Tauri frontend/backend (Desktop + iOS app), and YOLOv8 Autonomous Driving logic.

**Disclaimer:** This project is an independent modification and is **not affiliated with, endorsed by, or representative of the Freenove company**. We do not provide user support for Freenove hardware products. For hardware inquiries, please refer to the original manufacturer.

---

### Architecture & Planning
For a detailed breakdown of the microservices architecture, networking topologies, and planning/implementation trackers, please refer to the **[ARCHITECTURE.md](./ARCHITECTURE.md)** file in the project root. 

For instructions on deploying the mobile iOS app or troubleshooting Xcode errors (like `PhaseScriptExecution`), please read the **[iOS Deployment Guide](./docs/ios_deployment.md)**.

---

### Local Wi-Fi AP & Hardware Pairing
If the Raspberry Pi gateway cannot connect to a known Wi-Fi network on boot, it will automatically broadcast a local AP hotspot:
- **SSID:** `Picar-Vision-[last 4 of MAC]`
- **Password:** `picar-vision`

Once connected to the AP network via the desktop or iOS app, you MUST perform a hardware pairing to unlock telemetry and control.
**Pairing Procedure:** Toggle the car's physical motor power switch OFF and ON twice. The backend will detect the voltage drop on the ADC and unlock control.

---

### Mobile App Layout & Interaction

The mobile frontend is a unified interface built with Svelte and deployed natively to iOS using Tauri. It offers:
* **Live Video Feed:** Displayed directly in the main view.
* **Control Overlay:** Touch-based on-screen joysticks for driving and camera panning.
* **Menus:** 
  * Swipe down from the top handle to minimize the AI overlay.
  * Tap the **controller icon** in the top right for joystick calibration and adjusting motor/servo speed limits.
  * Tap the **gear icon** in the top right to open Settings (includes network status, IP address, battery levels, connected backend diagnostics, and debug telemetry).
* **Gamepad Support:** The app natively handles Bluetooth controllers (such as Xbox, PlayStation, and Nintendo Joy-Cons) via the Gamepad API.

---

### Gamepad Configuration

When a physical controller is connected, the UI will acknowledge it and allow you to drive the car with standard dual analog sticks. The button mappings adapt based on your controller type.

#### 1. Dual Joy-Cons (or Standard Gamepads like Xbox/PlayStation)
You get full mapping without compromises:
* **Left Stick:** Drive (Throttle/Steering)
* **Right Stick:** Camera Pan/Tilt
* **L2, R2, or L1:** Deadman Switch (Hold to activate AI modes)
* **Plus (+) / Start:** Toggles the Settings Menu
* **Minus (-) / Select:** Toggles the AI Controls Overlay
* **North (X/Y):** Toggle AI Autonomous Driving Mode
* **West (Y/X):** Toggle Sync Steer vs Auto Drive
* **Hold L1/R1 + East/South Face Buttons:** Adjust Motor/Servo Speeds dynamically

#### 2. Single Joy-Con (Left or Right, held sideways)
macOS treats a single Joy-Con as a "Micro Gamepad." Since there is only one stick, the layout is automatically remapped:
* **Analog Stick:** Drive (Throttle/Steering)
* **Face Buttons:** Acts as a D-Pad for Camera Pan/Tilt
* **Top Shoulder (L/R):** Toggle AI Autonomous Driving Mode
* **Bottom Trigger (ZL/ZR):** Toggle Sync Steer vs Auto Drive
* **SL or SR:** Deadman Switch
* **Plus or Minus (or Home/Capture):** Toggles Menus (Settings or AI Controls)
* **Hold SL/SR + Face Buttons:** Temporarily overrides the camera D-Pad to adjust Motor/Servo speeds.

---

### Copyright & Licensing
This software is provided "as is" and is released for educational and non-commercial purposes. Derived works must respect the open-source licenses of the underlying dependencies.