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
  let isDisconnecting = $state(false);

  // Virtual Controls State
  let keyW = $state(false);
  let keyA = $state(false);
  let keyS = $state(false);
  let keyD = $state(false);
  let motorSpeed = $state(100);
  let servoSensitivity = $state(50);
  
  // Gamepad State for UI (visual only)
  let gpDpadUp = $state(false);
  let gpDpadDown = $state(false);
  let gpDpadLeft = $state(false);
  let gpDpadRight = $state(false);
  let gpBtnSnap = $state(false);

  // Keyboard Arrow State (drives camera)
  let arrUp = $state(false);
  let arrDown = $state(false);
  let arrLeft = $state(false);
  let arrRight = $state(false);
  let centerCamera = $state(false);

  // Camera Absolute State (incremental)
  let pan = $state(0.0);
  let tilt = $state(0.0);
  let servoMode = $state<'incremental' | 'absolute'>('incremental');

  // Computed Joystick Position from WASD + Drag
  let dragX = $state(0.0);
  let dragY = $state(0.0);
  let joystickX = $derived(Math.max(-1, Math.min(1, (keyD ? 1 : 0) - (keyA ? 1 : 0) + dragX)));
  let joystickY = $derived(Math.max(-1, Math.min(1, (keyS ? 1 : 0) - (keyW ? 1 : 0) + dragY)));

  // Computed Camera Stick from Arrows + Drag
  let camDragX = $state(0.0);
  let camDragY = $state(0.0);
  let cameraX = $derived(Math.max(-1, Math.min(1, (arrRight ? 1 : 0) - (arrLeft ? 1 : 0) + camDragX)));
  let cameraY = $derived(Math.max(-1, Math.min(1, (arrDown ? 1 : 0) - (arrUp ? 1 : 0) + camDragY)));

  function handlePointerDown(e: PointerEvent, stick: 'L' | 'R') {
    const base = e.currentTarget as HTMLElement;
    base.setPointerCapture(e.pointerId);
    
    function move(e: PointerEvent) {
      const rect = base.getBoundingClientRect();
      const radius = rect.width / 2;
      const cx = rect.left + radius;
      const cy = rect.top + radius;
      
      let dx = (e.clientX - cx) / radius;
      let dy = (e.clientY - cy) / radius;
      
      const distance = Math.sqrt(dx*dx + dy*dy);
      if (distance > 1) {
        dx /= distance;
        dy /= distance;
      }
      
      if (stick === 'L') {
        dragX = dx;
        dragY = dy;
      } else {
        camDragX = dx;
        camDragY = dy;
      }
      sendKeyboardCommand();
    }
    
    function up(e: PointerEvent) {
      base.releasePointerCapture(e.pointerId);
      base.removeEventListener('pointermove', move);
      base.removeEventListener('pointerup', up);
      base.removeEventListener('pointercancel', up);
      
      if (stick === 'L') {
        dragX = 0;
        dragY = 0;
      } else {
        camDragX = 0;
        camDragY = 0;
      }
      sendKeyboardCommand();
    }
    
    base.addEventListener('pointermove', move);
    base.addEventListener('pointerup', up);
    base.addEventListener('pointercancel', up);
    
    move(e);
  }

  async function sendKeyboardCommand(includeCamera = false) {
    // Only send commands if connected, to avoid errors
    if (!connected) return;
    
    const scale = motorSpeed / 100.0;
    // Map to standard axes: throttle is positive up, steering is positive right
    const throttle = -joystickY * scale; // invert Y so W is positive throttle
    const steering = joystickX * scale;
    
    let payload: any = {
      type: 'command',
      throttle: throttle,
      steering: steering
    };

    if (servoMode === 'absolute') {
      const camScale = servoSensitivity / 100.0;
      payload.pan = cameraX * camScale;
      payload.tilt = -cameraY * camScale;
      if (centerCamera) {
        payload.pan = 0.0;
        payload.tilt = 0.0;
        centerCamera = false;
      }
    } else if (includeCamera) {
      payload.pan = pan;
      payload.tilt = tilt;
    }

    try {
      await invoke('send_pi_command', { command: JSON.stringify(payload) });
    } catch (e) {
      console.error("Failed to send command:", e);
    }
  }
  
  $effect(() => {
    invoke('update_settings', { motorSpeed, servoSensitivity }).catch(console.error);
  });

  function simulateKey(key: string, isDown: boolean) {
    const event = new KeyboardEvent(isDown ? 'keydown' : 'keyup', { key });
    if (isDown) handleKeydown(event);
    else handleKeyup(event);
  }

  function handleKeydown(e: KeyboardEvent) {
    const target = e.target as HTMLElement;
    if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA')) return;

    let changed = false;
    // WASD
    if (e.key === 'w' || e.key === 'W') { if (!keyW) { keyW = true; changed = true; } }
    if (e.key === 'a' || e.key === 'A') { if (!keyA) { keyA = true; changed = true; } }
    if (e.key === 's' || e.key === 'S') { if (!keyS) { keyS = true; changed = true; } }
    if (e.key === 'd' || e.key === 'D') { if (!keyD) { keyD = true; changed = true; } }
    
    // Arrow Keys (prevent scrolling and drive camera)
    if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
      e.preventDefault();
      if (e.key === 'ArrowUp') { arrUp = true; changed = true; }
      if (e.key === 'ArrowDown') { arrDown = true; changed = true; }
      if (e.key === 'ArrowLeft') { arrLeft = true; changed = true; }
      if (e.key === 'ArrowRight') { arrRight = true; changed = true; }
    }

    if (e.key === ' ') {
      e.preventDefault();
      centerCamera = true;
    }
    
    if (changed) sendKeyboardCommand();
  }

  function handleKeyup(e: KeyboardEvent) {
    const target = e.target as HTMLElement;
    if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA')) return;

    let changed = false;
    // WASD
    if (e.key === 'w' || e.key === 'W') { if (keyW) { keyW = false; changed = true; } }
    if (e.key === 'a' || e.key === 'A') { if (keyA) { keyA = false; changed = true; } }
    if (e.key === 's' || e.key === 'S') { if (keyS) { keyS = false; changed = true; } }
    if (e.key === 'd' || e.key === 'D') { if (keyD) { keyD = false; changed = true; } }
    
    // Arrow Keys
    if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
      e.preventDefault();
      if (e.key === 'ArrowUp') { arrUp = false; changed = true; }
      if (e.key === 'ArrowDown') { arrDown = false; changed = true; }
      if (e.key === 'ArrowLeft') { arrLeft = false; changed = true; }
      if (e.key === 'ArrowRight') { arrRight = false; changed = true; }
    }
    
    if (changed) sendKeyboardCommand();
  }

  async function toggleConnection() {
    if (!connected) {
      try {
        isDisconnecting = false;
        await invoke('connect_to_pi', { ip: ipAddress });
        connected = true;
      } catch (e) {
        console.error(e);
        alert(e);
      }
    } else {
      await disconnect();
    }
  }

  async function disconnect() {
    try {
      isDisconnecting = true;
      // Stop the motors before dropping the connection
      await invoke('send_pi_command', { command: JSON.stringify({
        type: 'command',
        throttle: 0.0,
        steering: 0.0
      })});
      await invoke('disconnect_from_pi');
      connected = false;
      videoBlobUrl = "";
    } catch (e) {
      console.error("Failed to disconnect:", e);
      isDisconnecting = false;
    }
  }

  let unlistens: Array<() => void> = [];
  let cameraInterval: any;

  onMount(async () => {
    // Incremental camera control loop
    cameraInterval = setInterval(() => {
      if (connected && servoMode === 'incremental' && (cameraX !== 0 || cameraY !== 0 || centerCamera)) {
        if (centerCamera) {
          pan = 0.0;
          tilt = 0.0;
          centerCamera = false;
        } else {
          // Multiply degrees step size based on sensitivity slider
          const step = (servoSensitivity / 100.0) * 0.05;
          pan += cameraX * step;
          tilt += -cameraY * step; // Inverted
          pan = Math.max(-1, Math.min(1, pan));
          tilt = Math.max(-1, Math.min(1, tilt));
        }
        sendKeyboardCommand(true);
      }
    }, 50);
    unlistens.push(await listen('ws-connected', () => { connected = true; isDisconnecting = false; }));
    unlistens.push(await listen('ws-disconnected', () => { connected = false; isDisconnecting = false; }));
    unlistens.push(await listen('controller-status', (event: any) => {
      if (event.payload.connected) {
        controllerStatus = event.payload.name || "Connected";
      } else {
        controllerStatus = "Not Detected";
      }
    }));
    unlistens.push(await listen('telemetry', (event: any) => {
      if (isDisconnecting) return;
      connected = true; // Recover connection state if UI hot-reloads
      
      distance = event.payload.distance?.toFixed(1) || "--";
      light = event.payload.left_light?.toFixed(1) || "--";
      
      if (event.payload.frame) {
        videoBlobUrl = "data:image/jpeg;base64," + event.payload.frame;
      }
    }));
    
    // Listen for Gamepad inputs from Rust for the UI
    unlistens.push(await listen('gamepad-input', (event: any) => {
      if (event.payload.button === 'DpadUp') gpDpadUp = event.payload.pressed;
      if (event.payload.button === 'DpadDown') gpDpadDown = event.payload.pressed;
      if (event.payload.button === 'DpadLeft') gpDpadLeft = event.payload.pressed;
      if (event.payload.button === 'DpadRight') gpDpadRight = event.payload.pressed;
      if (event.payload.button === 'RightTrigger2') gpBtnSnap = event.payload.pressed; // R1
    }));
    
    // Setup keyboard listeners
    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('keyup', handleKeyup);
  });
  
  onDestroy(() => {
    if (cameraInterval) clearInterval(cameraInterval);
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
      <input type="text" bind:value={ipAddress} placeholder="Raspberry Pi IP" disabled={connected} onkeydown={(e) => e.key === 'Enter' && !connected && toggleConnection()} />
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

      <!-- Horizontal Stats -->
      <div class="horizontal-stats">
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
      </div>
    </section>

    <!-- Telemetry Sidebar -->
    <aside class="telemetry-section">
      <!-- Virtual Controls -->
      <div class="glass-panel telemetry-card controls-card">
        <h3>Virtual Controls</h3>
        
        <div class="sliders">
          <div class="slider-group">
            <label for="motor-speed">Motor Speed: {motorSpeed}%</label>
            <input id="motor-speed" type="range" min="0" max="100" bind:value={motorSpeed} oninput={sendKeyboardCommand} />
          </div>
          <div class="slider-group">
            <label for="servo-sens">Servo Sensitivity: {servoSensitivity}%</label>
            <input id="servo-sens" type="range" min="0" max="100" bind:value={servoSensitivity} />
          </div>
          <div class="sensor-row" style="margin-bottom: 0; margin-top: 0.5rem;">
            <span style="font-size: 0.85rem; color: var(--text-secondary);">Camera Mode</span>
            <button class="mode-toggle" onclick={() => servoMode = (servoMode === 'incremental' ? 'absolute' : 'incremental')}>
              {servoMode === 'incremental' ? 'Incremental' : 'Absolute'}
            </button>
          </div>
        </div>

        <div class="keys-container">
          <!-- Driving (WASD) -->
          <div class="dpad-grid">
            <div></div>
            <div class="v-key" class:active={keyW} onpointerdown={() => simulateKey('w', true)} onpointerup={() => simulateKey('w', false)} onpointerleave={() => simulateKey('w', false)}>W</div>
            <div></div>
            <div class="v-key" class:active={keyA} onpointerdown={() => simulateKey('a', true)} onpointerup={() => simulateKey('a', false)} onpointerleave={() => simulateKey('a', false)}>A</div>
            <div class="v-key" class:active={keyS} onpointerdown={() => simulateKey('s', true)} onpointerup={() => simulateKey('s', false)} onpointerleave={() => simulateKey('s', false)}>S</div>
            <div class="v-key" class:active={keyD} onpointerdown={() => simulateKey('d', true)} onpointerup={() => simulateKey('d', false)} onpointerleave={() => simulateKey('d', false)}>D</div>
          </div>
          
          <!-- Virtual Joysticks Visual -->
          <div class="joysticks-row">
            <div class="joystick-wrapper">
              <span class="stick-label">L (WASD)</span>
              <div class="joystick-base" onpointerdown={(e) => handlePointerDown(e, 'L')} style="cursor: crosshair; touch-action: none;">
                <div class="joystick-stick" style="transform: translate({joystickX * 20}px, {joystickY * 20}px)"></div>
              </div>
            </div>
            <div class="joystick-wrapper">
              <span class="stick-label">R (Camera)</span>
              <div class="joystick-base" onpointerdown={(e) => handlePointerDown(e, 'R')} style="cursor: crosshair; touch-action: none;">
                <div class="joystick-stick" style="transform: translate({cameraX * 20}px, {cameraY * 20}px)"></div>
              </div>
            </div>
          </div>
          
          <!-- Servos (D-Pad & Snap) -->
          <div class="dpad-grid">
            <div></div>
            <div class="v-key" class:active={gpDpadUp || arrUp} onpointerdown={() => arrUp = true} onpointerup={() => arrUp = false} onpointerleave={() => arrUp = false}>▲</div>
            <div class="v-key btn-snap" class:active={gpBtnSnap || centerCamera} onpointerdown={() => centerCamera = true}>R1/SP</div>
            <div class="v-key" class:active={gpDpadLeft || arrLeft} onpointerdown={() => arrLeft = true} onpointerup={() => arrLeft = false} onpointerleave={() => arrLeft = false}>◀</div>
            <div class="v-key" class:active={gpDpadDown || arrDown} onpointerdown={() => arrDown = true} onpointerup={() => arrDown = false} onpointerleave={() => arrDown = false}>▼</div>
            <div class="v-key" class:active={gpDpadRight || arrRight} onpointerdown={() => arrRight = true} onpointerup={() => arrRight = false} onpointerleave={() => arrRight = false}>▶</div>
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

  .horizontal-stats {
    display: flex;
    gap: 1.5rem;
    margin-top: 1.5rem;
  }

  .horizontal-stats .telemetry-card {
    flex: 1;
    background: rgba(255, 255, 255, 0.03); /* slightly different background for nested cards */
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
    box-sizing: border-box;
    margin: 0;
    accent-color: var(--accent-primary);
    cursor: pointer;
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
    cursor: pointer;
    user-select: none;
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

  .mode-toggle {
    padding: 0.25rem 0.75rem;
    font-size: 0.75rem;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: var(--text-primary);
  }
  .mode-toggle:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .joysticks-row {
    display: flex;
    flex-direction: row;
    gap: 2.5rem;
    justify-content: center;
    width: 100%;
    margin: 0.5rem 0;
  }

  .joystick-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .stick-label {
    font-size: 0.8rem;
    color: var(--text-secondary);
    font-weight: 600;
  }

  .joystick-base {
    width: 70px;
    height: 70px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.05);
    border: 2px solid rgba(255, 255, 255, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .joystick-stick {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: var(--accent-primary);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3), inset 0 2px 4px rgba(255, 255, 255, 0.3);
    transition: transform 0.1s ease-out;
  }
</style>
