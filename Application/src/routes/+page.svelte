<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  // Core Dashboard State
  let connected = $state(false);
  let ipAddress = $state("10.0.0.X");
  
  let distance = $state("--");
  let light = $state("--");
  let controllerStatus = $state("Not Detected");
  let videoBlobUrl = $state("");

  // Virtual Controls State
  let keyW = $state(false);
  let keyA = $state(false);
  let keyS = $state(false);
  let keyD = $state(false);
  let motorSpeed = $state(100);
  let servoSensitivity = $state(50);
  
  // Gamepad State for UI (updated from Rust events)
  let dpadUp = $state(false);
  let dpadDown = $state(false);
  let dpadLeft = $state(false);
  let dpadRight = $state(false);
  let btnSnap = $state(false);

  // Computed Joystick Position from WASD (-1.0 to 1.0)
  let joystickX = $derived((keyD ? 1 : 0) - (keyA ? 1 : 0));
  let joystickY = $derived((keyS ? 1 : 0) - (keyW ? 1 : 0)); // Note: Y is inverted for screen (up is negative Y)

  async function sendKeyboardCommand() {
    // Only send commands if connected, to avoid errors
    if (!connected) return;
    
    const scale = motorSpeed / 100.0;
    // Map to standard axes: throttle is positive up, steering is positive right
    const throttle = -joystickY * scale; // invert Y so W is positive throttle
    const steering = joystickX * scale;
    
    try {
      await invoke('send_pi_command', { command: JSON.stringify({
        type: 'command',
        throttle: throttle,
        steering: steering
      })});
    } catch (e) {
      console.error("Failed to send command:", e);
    }
  }
  
  $effect(() => {
    invoke('update_settings', { motorSpeed, servoSensitivity }).catch(console.error);
  });

  function handleKeydown(e: KeyboardEvent) {
    let changed = false;
    if (e.key === 'w' || e.key === 'W') { if (!keyW) { keyW = true; changed = true; } }
    if (e.key === 'a' || e.key === 'A') { if (!keyA) { keyA = true; changed = true; } }
    if (e.key === 's' || e.key === 'S') { if (!keyS) { keyS = true; changed = true; } }
    if (e.key === 'd' || e.key === 'D') { if (!keyD) { keyD = true; changed = true; } }
    if (changed) sendKeyboardCommand();
  }

  function handleKeyup(e: KeyboardEvent) {
    let changed = false;
    if (e.key === 'w' || e.key === 'W') { if (keyW) { keyW = false; changed = true; } }
    if (e.key === 'a' || e.key === 'A') { if (keyA) { keyA = false; changed = true; } }
    if (e.key === 's' || e.key === 'S') { if (keyS) { keyS = false; changed = true; } }
    if (e.key === 'd' || e.key === 'D') { if (keyD) { keyD = false; changed = true; } }
    if (changed) sendKeyboardCommand();
  }

  async function toggleConnection() {
    if (!connected) {
      try {
        await invoke('connect_to_pi', { ip: ipAddress });
        connected = true;
      } catch (e) {
        console.error(e);
        alert(e);
      }
    } else {
      // Disconnect is not fully implemented in the backend yet.
      // For now, we will just alert the user.
      alert("Disconnect not implemented yet! Please restart the app.");
    }
  }

  let unlistens: Array<() => void> = [];

  onMount(async () => {
    unlistens.push(await listen('ws-connected', () => { connected = true; }));
    unlistens.push(await listen('ws-disconnected', () => { connected = false; }));
    unlistens.push(await listen('controller-status', (event: any) => {
      if (event.payload.connected) {
        controllerStatus = event.payload.name || "Connected";
      } else {
        controllerStatus = "Not Detected";
      }
    }));
    unlistens.push(await listen('telemetry', (event: any) => {
      distance = event.payload.distance?.toFixed(1) || "--";
      light = event.payload.left_light?.toFixed(1) || "--";
      
      if (event.payload.frame) {
        videoBlobUrl = "data:image/jpeg;base64," + event.payload.frame;
      }
    }));
    
    // Listen for Gamepad inputs from Rust for the UI
    unlistens.push(await listen('gamepad-input', (event: any) => {
      if (event.payload.button === 'DpadUp') dpadUp = event.payload.pressed;
      if (event.payload.button === 'DpadDown') dpadDown = event.payload.pressed;
      if (event.payload.button === 'DpadLeft') dpadLeft = event.payload.pressed;
      if (event.payload.button === 'DpadRight') dpadRight = event.payload.pressed;
      if (event.payload.button === 'RightTrigger2') btnSnap = event.payload.pressed; // R1
    }));
    
    // Setup keyboard listeners
    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('keyup', handleKeyup);
  });
  
  onDestroy(() => {
    unlistens.forEach(u => u());
    window.removeEventListener('keydown', handleKeydown);
    window.removeEventListener('keyup', handleKeyup);
  });
</script>

<main class="dashboard">
  <header class="glass-panel">
    <div class="logo-area">
      <h1>Picar-Vision</h1>
      <span class="badge" class:active={connected}>
        {connected ? 'Connected' : 'Disconnected'}
      </span>
    </div>
    
    <div class="controls-area">
      <input type="text" bind:value={ipAddress} placeholder="Raspberry Pi IP" disabled={connected} />
      <button onclick={toggleConnection} class={connected ? 'btn-danger' : 'btn-primary'}>
        {connected ? 'Disconnect' : 'Connect'}
      </button>
    </div>
  </header>

  <div class="content-grid">
    <!-- Video Feed Panel -->
    <section class="video-section glass-panel">
      <div class="panel-header">
        <h2>Live Feed</h2>
      </div>
      <div class="video-container" class:offline={!connected}>
        {#if connected}
          {#if videoBlobUrl}
            <img class="video-stream" src={videoBlobUrl} alt="Live MJPEG stream" />
          {:else}
            <div class="stream-placeholder">Video Stream Active... Waiting for frames.</div>
          {/if}
        {:else}
          <div class="stream-placeholder">Waiting for connection...</div>
        {/if}
      </div>
    </section>

    <!-- Telemetry Sidebar -->
    <aside class="telemetry-section">
      <div class="glass-panel telemetry-card">
        <h3>Sensors</h3>
        <div class="sensor-row">
          <span>Distance</span>
          <span class="value">{distance} cm</span>
        </div>
        <div class="sensor-row">
          <span>Light</span>
          <span class="value">{light} / 255</span>
        </div>
      </div>
      
      <div class="glass-panel telemetry-card">
        <h3>Controller Status</h3>
        <div class="sensor-row">
          <span>JoyCon</span>
          <span class="value" class:disconnected={controllerStatus === "Not Detected"}>{controllerStatus}</span>
        </div>
      </div>

      <!-- Virtual Controls -->
      <div class="glass-panel telemetry-card controls-card">
        <h3>Virtual Controls</h3>
        
        <div class="sliders">
          <div class="slider-group">
            <label for="motor-speed">Motor Speed: {motorSpeed}%</label>
            <input id="motor-speed" type="range" min="0" max="100" bind:value={motorSpeed} onchange={sendKeyboardCommand} />
          </div>
          <div class="slider-group">
            <label for="servo-sens">Servo Sensitivity: {servoSensitivity}%</label>
            <input id="servo-sens" type="range" min="0" max="100" bind:value={servoSensitivity} />
          </div>
        </div>

        <div class="keys-container">
          <!-- Driving (WASD) -->
          <div class="dpad-grid">
            <div></div>
            <div class="v-key" class:active={keyW}>W</div>
            <div></div>
            <div class="v-key" class:active={keyA}>A</div>
            <div class="v-key" class:active={keyS}>S</div>
            <div class="v-key" class:active={keyD}>D</div>
          </div>
          
          <!-- Virtual Joystick Visual -->
          <div class="joystick-wrapper">
            <div class="joystick-base">
              <div class="joystick-stick" style="transform: translate({joystickX * 20}px, {joystickY * 20}px)"></div>
            </div>
          </div>
          
          <!-- Servos (D-Pad & Snap) -->
          <div class="dpad-grid">
            <div></div>
            <div class="v-key" class:active={dpadUp}>▲</div>
            <div class="v-key btn-snap" class:active={btnSnap}>R1</div>
            <div class="v-key" class:active={dpadLeft}>◀</div>
            <div class="v-key" class:active={dpadDown}>▼</div>
            <div class="v-key" class:active={dpadRight}>▶</div>
          </div>
        </div>
      </div>
    </aside>
  </div>
</main>

<style>
  .dashboard {
    padding: 1.5rem;
    height: 100vh;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
  }

  .logo-area {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  h1 {
    font-size: 1.5rem;
    background: linear-gradient(135deg, #60a5fa, #a78bfa);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .badge {
    padding: 0.25rem 0.75rem;
    border-radius: 999px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    background: rgba(239, 68, 68, 0.2);
    color: var(--accent-danger);
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .badge.active {
    background: rgba(16, 185, 129, 0.2);
    color: var(--accent-success);
    border: 1px solid rgba(16, 185, 129, 0.3);
  }

  .controls-area {
    display: flex;
    gap: 1rem;
  }

  .btn-danger {
    background: var(--accent-danger);
  }
  .btn-danger:hover {
    background: #dc2626;
  }

  .content-grid {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 1.5rem;
    flex: 1;
    min-height: 0;
  }

  .video-section {
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
  }

  .panel-header {
    margin-bottom: 1rem;
  }

  .video-container {
    flex: 1;
    background: rgba(0, 0, 0, 0.4);
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .stream-placeholder {
    color: var(--text-secondary);
    font-weight: 500;
    letter-spacing: 0.05em;
  }

  .video-stream {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .telemetry-section {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .telemetry-card {
    padding: 1.5rem;
  }

  .telemetry-card h3 {
    font-size: 1.1rem;
    color: var(--text-secondary);
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .sensor-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
    font-weight: 500;
  }

  .sensor-row:last-child {
    margin-bottom: 0;
  }

  .value {
    color: var(--accent-primary);
    font-family: monospace;
    font-size: 1.1rem;
  }

  .disconnected {
    color: var(--accent-danger);
    font-size: 0.9rem;
    font-family: inherit;
  }

  .controls-card {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .sliders {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .slider-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .slider-group label {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .slider-group input[type=range] {
    width: 100%;
    accent-color: var(--accent-primary);
  }

  .keys-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
  }

  .dpad-grid {
    display: grid;
    grid-template-columns: repeat(3, 40px);
    grid-template-rows: repeat(2, 40px);
    gap: 6px;
    justify-content: center;
  }

  .v-key {
    width: 40px;
    height: 40px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-secondary);
    transition: all 0.1s;
  }

  .v-key.active {
    background: var(--accent-primary);
    color: #fff;
    box-shadow: 0 0 10px rgba(96, 165, 250, 0.5);
    transform: scale(0.95);
  }

  .btn-snap {
    font-size: 0.75rem;
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.3);
  }

  .btn-snap.active {
    background: var(--accent-danger);
    box-shadow: 0 0 10px rgba(239, 68, 68, 0.5);
  }

  .joystick-wrapper {
    display: flex;
    justify-content: center;
  }

  .joystick-base {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.05);
    border: 2px solid rgba(255, 255, 255, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .joystick-stick {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    background: var(--accent-primary);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3), inset 0 2px 4px rgba(255, 255, 255, 0.3);
    transition: transform 0.1s ease-out;
  }
</style>
