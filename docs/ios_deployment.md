# Picar-Vision iOS Deployment Guide

To deploy the Picar-Vision app directly to an iOS device, you must use Xcode to compile and sign the app. Follow the instructions below to configure your device and build the app.

## Prerequisites

1.  **Xcode**: Install Xcode from the Mac App Store.
2.  **Apple ID**: Ensure you have an Apple ID (a paid Developer account is not required for personal local deployment, but a free personal account is needed to generate signing certificates).
3.  **Dependencies**: Rust, Node.js, and CocoaPods must be installed on your Mac. If you have run `npm run tauri ios init`, these are likely already set up.

## Steps to Deploy

### 1. Open the Xcode Project
The Tauri CLI generates an Xcode workspace. Navigate to the project directory and open it:
```bash
cd Application/src-tauri/gen/apple
open tauri-app.xcodeproj
```

### 2. Configure Signing and Capabilities
Before you can run the app on a physical device, you must configure code signing.

1.  In Xcode, click on the **tauri-app_iOS** project in the left navigator pane.
2.  Select the **tauri-app_iOS** target under the "Targets" section.
3.  Navigate to the **Signing & Capabilities** tab.
4.  Check the box for **"Automatically manage signing"**.
5.  In the **Team** dropdown, select your Apple ID (e.g., "John Doe (Personal Team)").
    - *Note: If you do not see your account, go to Xcode -> Settings -> Accounts and sign in with your Apple ID.*
6.  The **Bundle Identifier** is pre-configured as `com.picar.vision`. If you encounter a conflict, append a unique string to the end (e.g., `com.picar.vision.myname`).

### 3. Trust the Developer Certificate on your Device
If this is your first time deploying an app to your device using a free Apple Developer account, you must trust the certificate on the device itself.

1.  Connect your iOS device to your Mac via USB.
2.  At the top of the Xcode window, click the device selector (it usually defaults to an iOS Simulator) and select your physical iOS device.
3.  Click the **Play** button (Run) in the top-left corner of Xcode.
4.  The app will install on your device, but it may fail to launch with an "Untrusted Developer" error.
5.  On your iOS device, go to **Settings > General > VPN & Device Management**.
6.  Under "Developer App", tap your Apple ID email.
7.  Tap "Trust [Your Apple ID]" and confirm.

### 4. Enable Developer Mode (iOS 16+)
If you are running iOS 16 or later, you must enable Developer Mode on your device to run locally installed apps.

1.  On your iOS device, go to **Settings > Privacy & Security**.
2.  Scroll down to the bottom and tap **Developer Mode**.
3.  Toggle **Developer Mode** on.
4.  Your device will prompt you to restart. After restarting, unlock your device and tap **Turn On** when prompted.

### 5. Run the App
With signing configured, the certificate trusted, and Developer Mode enabled:

1.  In Xcode, ensure your device is selected.
2.  Click the **Play** button (or press `Cmd + R`) to build and run the app.
3.  The app will compile, install, and automatically launch on your iOS device in Landscape mode.

## Network Requirements

- **Local Network Access**: Since the Picar-Vision app communicates with the Raspberry Pi over WebSockets and fetches MJPEG streams locally, your iOS device **must be connected to the same Wi-Fi network** as the Raspberry Pi.
- **App Permissions**: On its first launch, iOS will ask for permission to "find and connect to devices on your local network." You **must** allow this, or the app will not be able to communicate with the Pi.

## Future Updates

If you change the Svelte frontend code (`Application/src/routes/+page.svelte` or CSS) or Rust backend code, you can easily deploy the update:
```bash
npm run tauri ios dev
```
Alternatively, just rebuild from Xcode after ensuring your frontend is built (`npm run build`).

## Troubleshooting

### Error: `Command PhaseScriptExecution failed with a nonzero exit code` or `Operation not permitted (os error 1)`

If your build fails during the `PhaseScriptExecution Build Rust Code` step (specifically with `error: failed to determine package fingerprint for build script`), it is likely due to **Xcode 15's new User Script Sandboxing** feature.

When this is enabled, Xcode restricts custom build scripts (like Tauri's Rust compilation step) from accessing files outside of explicit input/output bounds. Since `cargo` needs to read the entire project directory and `project.pbxproj`, it gets blocked by `sandbox-exec`.

**The Fix:**
1. Open the project in Xcode (`open -a Xcode gen/apple/tauri-app.xcodeproj`)
2. Click on the `tauri-app` project file in the Project Navigator (left sidebar).
3. Select the `tauri-app_iOS` target.
4. Go to the **Build Settings** tab.
5. Search for `ENABLE_USER_SCRIPT_SANDBOXING` (or "User Script Sandboxing").
6. Change the value from `Yes` to `No`.
7. Clean your build folder (⇧⌘K) and run the build again.

### Error: `No profiles for 'com.picar.vision' were found`

When building the Tauri iOS app via Xcode, you may encounter a `PhaseScriptExecution` error during the build phase. This happens because Xcode's GUI application does not inherit the same `PATH` variables as your terminal, meaning it often cannot find `node`, `npm`, or `cargo` if they were installed via tools like `nvm` or `rustup`.

**The Solution:**
To ensure Xcode inherits your terminal's environment variables (including Node.js and Rust binaries), you should always open the project using the terminal rather than clicking the `.xcodeproj` file in Finder or opening it from the Xcode GUI.

1. Close Xcode.
2. Open your terminal application (e.g., iTerm2 or Terminal).
3. Navigate to the project root:
   ```bash
   cd ~/Developer/picar-vision/Application
   ```
4. Open the Xcode workspace using the `open -a` command:
   ```bash
   open -a Xcode src-tauri/gen/apple/tauri-app.xcodeproj
   ```
5. Once Xcode opens, clean the build folder by going to **Product > Clean Build Folder** (or press `Cmd+Shift+K`).
6. Click **Run** again.

*Alternative*: If this still fails, you can symlink `node` into a location Xcode can always see:
```bash
sudo ln -s $(which node) /usr/local/bin/node
```
