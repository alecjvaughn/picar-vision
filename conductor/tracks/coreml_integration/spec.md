# Specification: CoreML Native iOS Integration

## Overview
The `ort` (ONNX Runtime) crate cannot dynamically load iOS shared libraries for the ONNX C API natively. Instead of relying on ONNX on iOS, we will build a custom Tauri plugin written in Swift that leverages Apple's `Vision` framework and CoreML to run inference directly on the Apple Neural Engine (ANE).

## Functional Requirements
1. **Model Conversion:** Convert the `yolov8n.onnx` model to an Apple CoreML `.mlpackage` format using `coremltools`.
2. **Swift Tauri Plugin:** Develop a Tauri v2 mobile plugin containing Swift code that utilizes the `Vision` framework (`VNCoreMLModel`, `VNCoreMLRequest`) to execute the model.
3. **Data Bridging:** Bridge image data (MJPEG frames) from Rust or JavaScript into the Swift plugin for inference.
4. **Result Parsing:** Parse the CoreML tensor outputs (bounding boxes, class indices, confidence scores) in Swift and pass the array back to the Tauri frontend via IPC.
5. **Conditional Compilation:** Ensure the Rust codebase uses `ort` on macOS (`#[cfg(target_os = "macos")]`) and the new CoreML plugin on iOS (`#[cfg(target_os = "ios")]`).

## Non-Functional Requirements
- **Performance:** Inference must run at > 15 FPS on an A13 Bionic chip.
- **Bundle Size:** The `.mlpackage` should be compressed or quantized (INT8/FP16) to keep the app bundle under 30MB.
