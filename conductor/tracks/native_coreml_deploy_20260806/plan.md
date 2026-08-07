# Implementation Plan: Native CoreML & Deployment Fixes

## Phase 1: Native Xcode Deployment Resolution
- [ ] Task: Investigate and resolve Xcode `PhaseScriptExecution` `npm` path errors.
    - [ ] Sub-task: Modify the Xcode build phase script or provide permanent system-level symlinks to ensure Xcode correctly inherits the Node/npm PATH.
- [ ] Task: Ensure App Icon and Product Name are configured correctly for Xcode.
    - [ ] Sub-task: Verify `Info.plist` and `Assets.xcassets` are correctly linked and recognized by Xcode's native build process.
- [ ] Task: Test native Xcode deployment.
    - [ ] Sub-task: Deploy the app to the tethered iPhone SE 2020 directly using Xcode's 'Play' button.

## Phase 2: iOS Network Permissions
- [ ] Task: Resolve local network blocks.
    - [ ] Sub-task: Add `NSLocalNetworkUsageDescription` and `NSBonjourServices` to `Info.plist` if required for WebSocket/Vite dev server connections.
    - [ ] Sub-task: Verify the iOS app successfully prompts for network access and connects to the backend.

## Phase 3: CoreML Integration & Fallback Strategies
- [x] Task: Conduct Research on CoreML integration.
    - [x] Sub-task: Perform targeted research (via NotebookLM or web) on `.mlmodelc` bundling with Tauri and memory limits of ANE on A13 Bionic.
- [x] Task: Implement ANE Fallback in Swift Plugin.
    - [x] Sub-task: Update `CoreMLPlugin.swift` to attempt loading with `MLComputeUnits.all` (ANE).
    - [x] Sub-task: Implement robust `catch` block to fallback to `MLComputeUnits.cpuAndGPU` if ANE compilation or loading fails.
- [x] Task: Model Packaging & Optimization.
    - [x] Sub-task: Package the `.mlmodelc` inside the iOS app bundle using the most optimal strategy for the iPhone SE 2020 storage/memory constraints.
- [x] Task: Test native CoreML inference.
    - [x] Sub-task: Verify YOLOv8 inference runs directly on the device using the Swift CoreML plugin without crashing.
