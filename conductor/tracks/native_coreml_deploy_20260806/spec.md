# Specification: Native CoreML & Deployment Fixes

## Overview
This track focuses on refining the Swift/Rust/Tauri iOS app to run YOLOv8 object detection natively via CoreML on the iPhone SE 2020. It includes implementing robust hardware fallback strategies for the Apple Neural Engine (ANE) and resolving ongoing deployment and packaging issues (Xcode build script failures, missing icons, and network permission blocks). The implementation will leverage NotebookLM research to identify the most compatible model format and deployment strategy.

## Functional Requirements
- **CoreML Fallback Strategy:** The native Swift CoreML plugin must attempt to load the model on the Neural Engine (ANE) first, and gracefully fall back to CPU/GPU execution if ANE compilation fails or is unsupported.
- **Model Packaging & Storage:** Evaluate model storage and memory requirements to ensure the iOS app is highly optimized for the iPhone SE 2020. Implement the most successful strategy for bundling or loading the `.mlmodelc` assets based on NotebookLM research.
- **Natural Xcode Deployment:** The Xcode project must be fully configured to build and deploy to the tethered device *naturally* using the Xcode interface (the "Play" button), eliminating the need for terminal-based builds (`npm run tauri ios build`) or manually dragging `.ipa` files.
- **Network Permissions:** Resolve local network permission blocks preventing the iOS app from connecting to the Tauri development server and Raspberry Pi WebSocket backend.
- **App Configuration:** Fix missing app icons and ensure the `productName` is correctly applied to the iOS Xcode project directly, allowing Xcode to handle bundling.
- **Build Reliability:** Resolve Xcode `PhaseScriptExecution` environment path errors (like `npm: command not found`) so Xcode can successfully trigger the Tauri Rust build internally.

## Non-Functional Requirements
- **Performance:** Inference on the fallback CPU/GPU configuration must maintain reasonable framerates for object tracking within the device's memory limits.
- **Developer Experience:** Deployment must feel completely native to Xcode.

## Acceptance Criteria
- [ ] CoreML plugin successfully runs inference on the iPhone SE 2020 without crashing during ANE compilation.
- [ ] The app automatically falls back to CPU/GPU if the Neural Engine is incompatible.
- [ ] The iOS app builds and deploys to the device successfully *directly from Xcode* (Play button) without `PhaseScriptExecution` PATH errors.
- [ ] Manual terminal builds and manual `.ipa` installations are no longer required for device testing.
- [ ] The iOS app requests and receives local network permissions.
- [ ] The app installs with the correct icon and name via standard Xcode deployment.
- [ ] The `.mlmodelc` model is successfully located and loaded by the app using the selected packaging strategy, respecting memory limits.

## Out of Scope
- Modifying the Raspberry Pi Python backend.
- Modifying the Svelte frontend UI (beyond displaying potential CoreML debug errors).
