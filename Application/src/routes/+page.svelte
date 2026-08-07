<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  // Core Dashboard State
  let connected = $state(false);
  let ipAddress = $state("raspi3.local");
  
  let distance = $state("--");
  let light = $state("--");
  let controllerStatus = $state("Not Detected");
  let controllerBattery = $state("Unknown");
  let controllerType = $state("Standard");
  let videoBlobUrl = $state("");
  let isDisconnecting = $state(false);
  let connectionError = $state<string | null>(null);

  let expandedSections = $state<Record<string, boolean>>({
    connection: true,
    controller: true,
    uiPreferences: true,
    aiTracking: true,
    aiDebug: false
  });
  
  let allExpanded = $derived(Object.values(expandedSections).every(v => v));

  function toggleAllSettings(expand: boolean) {
    for (let k in expandedSections) {
      expandedSections[k] = expand;
    }
  }

  // UI States
  let showSettings = $state(false);
  let showRobotControls = $state(false);
  let uiOpacity = $state(0.7);
  let showTelemetry = $state(true);

  // Autonomous Mode State
  let isAutonomous = $state(false);
  let targetClass = $state('person');
  let boundingBoxes = $state<any[]>([]);
  let deadmanActive = $state(false);

  let allTrackingTargets = $state([
    { id: 'none', label: 'FREE ROAM', active: true },
    { id: 'person', label: 'PERSON', active: true },
    { id: 'hand', label: 'HAND', active: false },
    { id: 'sports ball', label: 'BALL', active: true },
    { id: 'bottle', label: 'BOTTLE', active: true },
    { id: 'car', label: 'CAR', active: true },
    { id: 'chair', label: 'CHAIR', active: true },
    { id: 'dog', label: 'DOG', active: false },
    { id: 'cat', label: 'CAT', active: false },
  ]);

  let trackingTargets = $derived(allTrackingTargets.filter(t => t.active));

  let draggedIndex = $state<number | null>(null);

  function handleDragStart(e: DragEvent, index: number) {
    draggedIndex = index;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', index.toString());
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = 'move';
  }

  function handleDrop(e: DragEvent, dropIndex: number) {
    e.preventDefault();
    if (draggedIndex === null || draggedIndex === dropIndex) return;
    const item = allTrackingTargets.splice(draggedIndex, 1)[0];
    allTrackingTargets.splice(dropIndex, 0, item);
    draggedIndex = null;
  }

  let wheelContainer = $state<HTMLElement | null>(null);
  let scrollTimeout = $state<any>(null);

  function handleWheelScroll() {
    if (!wheelContainer) return;
    clearTimeout(scrollTimeout);
    scrollTimeout = setTimeout(() => {
      const containerCenter = wheelContainer.getBoundingClientRect().left + (wheelContainer.clientWidth / 2);
      let closestElement = null;
      let minDistance = Infinity;
      Array.from(wheelContainer.children).forEach(child => {
        if (child.hasAttribute('data-target')) {
          const childCenter = child.getBoundingClientRect().left + (child.clientWidth / 2);
          const distance = Math.abs(containerCenter - childCenter);
          if (distance < minDistance) {
            minDistance = distance;
            closestElement = child;
          }
        }
      });
      if (closestElement) {
        targetClass = closestElement.getAttribute('data-target');
      }
    }, 150);
  }

  function scrollToTarget(target) {
     if (!wheelContainer) return;
     const el = wheelContainer.querySelector(`[data-target="${target}"]`);
     if (el) el.scrollIntoView({ behavior: 'smooth', inline: 'center', block: 'nearest' });
  }

  $effect(() => {
     if (targetClass && wheelContainer) {
       // Wait a tick for rendering
       setTimeout(() => scrollToTarget(targetClass), 10);
     }
  });

  // Sync autonomous state to Rust
  $effect(() => {
    invoke('set_autonomous_mode', { enabled: isAutonomous, targetClass: targetClass }).catch(console.error);
    invoke('set_deadman_state', { active: deadmanActive }).catch(console.error);
  });

  // Virtual Controls State
  let keyW = $state(false);
  let keyA = $state(false);
  let keyS = $state(false);
  let keyD = $state(false);
  let motorSpeed = $state(100);
  let servoSensitivity = $state(50);
  let viewportTurn = $state(false);

  // Sync settings to Rust controller
  $effect(() => {
    invoke('update_settings', { 
      motorSpeed: Number(motorSpeed), 
      servoSensitivity: Number(servoSensitivity),
      viewportTurn 
    }).catch(console.error);
    
    invoke('set_follow_mode', { active: viewportTurn }).catch(console.error);
    invoke('set_free_roam', { active: viewportTurn }).catch(console.error);
  });
  
  // Calibration State
  let showCalibration = $state(false);
  
  // Gamepad Switcher State
  let showGamepadModal = $state(false);
  let gamepads = $state<any[]>([]);

  // Auto-hide virtual controls logic
  let isGamepadConnected = $derived(controllerStatus !== "Not Detected");

  let calLxCenter = $state(0);
  let calLyCenter = $state(0);
  let calRxCenter = $state(0);
  let calRyCenter = $state(0);
  let calLxDeadzone = $state(200);
  let calLyDeadzone = $state(200);
  let calRxDeadzone = $state(200);
  let calRyDeadzone = $state(200);

  function saveCalibration() {
    invoke('update_calibration', {
      lxC: Number(calLxCenter), lyC: Number(calLyCenter), rxC: Number(calRxCenter), ryC: Number(calRyCenter),
      lxD: Number(calLxDeadzone), lyD: Number(calLyDeadzone), rxD: Number(calRxDeadzone), ryD: Number(calRyDeadzone)
    }).catch(console.error);
    showCalibration = false;
  }
  
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
  
  let ledActive = $state(false);
  let buzzerActive = $state(false);
  
  let aiDebugInfo = $state('');

  // Svelte 5 equivalent of stores
  let settingsOpen = $state(false);
  let aiControlsMinimized = $state(false);
  let swipeStartY = $state(0);
  
  function handleSwipeStart(e: PointerEvent) {
    swipeStartY = e.clientY;
  }
  
  function handleSwipeEnd(e: PointerEvent) {
    const deltaY = e.clientY - swipeStartY;
    if (deltaY > 20) {
      aiControlsMinimized = true; // swipe down
    } else if (deltaY < -20) {
      aiControlsMinimized = false; // swipe up
    } else {
      aiControlsMinimized = !aiControlsMinimized; // tap to toggle
    }
  }
  // Camera Absolute State
  let pan = $state(0.0);
  let tilt = $state(0.0);
  // FIXME: Incremental mode is currently disabled due to servo spasming issues
  let servoMode = $state<'incremental' | 'absolute'>('absolute');

  function setVArrow(dir: 'Up'|'Down'|'Left'|'Right', state: boolean) {
    if (dir === 'Up') arrUp = state;
    if (dir === 'Down') arrDown = state;
    if (dir === 'Left') arrLeft = state;
    if (dir === 'Right') arrRight = state;
    sendKeyboardCommand();
  }

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

  // Multi-touch tracking for joysticks
  let activePointers: Record<number, 'L' | 'R'> = {};

  function handlePointerDown(e: PointerEvent, stick: 'L'|'R') {
    activePointers[e.pointerId] = stick;
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
    updateJoystick(e, stick);
  }

  function handlePointerMove(e: PointerEvent) {
    const stick = activePointers[e.pointerId];
    if (stick) {
      updateJoystick(e, stick);
    }
  }

  function handlePointerUp(e: PointerEvent) {
    const stick = activePointers[e.pointerId];
    if (stick) {
      (e.target as HTMLElement).releasePointerCapture(e.pointerId);
      delete activePointers[e.pointerId];
      if (stick === 'L') { dragX = 0; dragY = 0; }
      if (stick === 'R') { camDragX = 0; camDragY = 0; centerCamera = true; }
      sendKeyboardCommand();
    }
  }

  function updateJoystick(e: PointerEvent, stick: 'L'|'R') {
    const base = (e.currentTarget as HTMLElement);
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

  async function sendKeyboardCommand(includeCamera = false, isManualInput = false) {
    // Only send commands if connected, to avoid errors
    if (!connected) return;
    
    // Disable autonomous mode if a manual steering/throttle command is sent
    if (isManualInput && isAutonomous) {
      isAutonomous = false;
    }
    
    const scale = Number(motorSpeed) / 100.0;
    // Map to standard axes: throttle is positive up, steering is positive right
    const throttle = -joystickY * scale; // invert Y so W is positive throttle
    let steering = joystickX * scale;
    
    let payload: any = {
      type: 'command',
      throttle: throttle,
      steering: steering
    };

    if (servoMode === 'absolute') {
      const camScale = Number(servoSensitivity) / 100.0;
      payload.pan = cameraX * camScale;
      payload.tilt = -cameraY * camScale;

      if (gpDpadLeft || arrLeft) payload.pan = -camScale;
      if (gpDpadRight || arrRight) payload.pan = camScale;
      if (gpDpadUp || arrUp) payload.tilt = camScale;
      if (gpDpadDown || arrDown) payload.tilt = -camScale;

      if (centerCamera) {
        payload.pan = 0.0;
        payload.tilt = 0.0;
        centerCamera = false;
      }
    } else if (includeCamera) {
      payload.pan = pan;
      payload.tilt = tilt;
    }

    if (viewportTurn) {
      steering = (servoMode === 'absolute') ? payload.pan : pan;
      payload.steering = steering;
    }
    
    payload.led = ledActive ? "blink" : "off";
    payload.buzzer = buzzerActive;

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
    
    if (changed) sendKeyboardCommand(false, true);
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
        connectionError = null;
        localStorage.setItem('picar-ip', ipAddress);
        await invoke('connect_to_pi', { ip: ipAddress });
        connected = true;
      } catch (e) {
        console.error("Failed to connect:", e);
        connectionError = typeof e === 'string' ? e : JSON.stringify(e);
        connected = false;
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
  let animationFrameId: number;
  let lastFrameTime: number;
  let activeGamepadId: string | null = null;
  let autoSelectedPadId: string | null = null;
  let lastWebGamepadState = { lx: 0, ly: 0, rx: 0, ry: 0, up: false, down: false, left: false, right: false, r1: false, deadman: false, btnNorth: false, btnSouth: false, btnEast: false, btnWest: false, btnPlus: false, btnMinus: false, btnHome: false, btnCapture: false, aiToggle: false, syncToggle: false };

  function setGamepadMode(id: string | null) {
    activeGamepadId = id;
  }

  function toggleAutoSelect(checked: boolean) {
    if (checked) {
      setGamepadMode(null);
    } else {
      setGamepadMode(autoSelectedPadId || (gamepads[0] ? gamepads[0].id : 'none'));
    }
  }

  function pollWebGamepad() {
    const pads = navigator.getGamepads 
      ? (Array.from(navigator.getGamepads()).filter(p => p !== null && p.connected) as Gamepad[]) 
      : [];
    
    // Smart Auto-Select: switch to whatever gamepad has activity
    if (activeGamepadId === null) {
      if (autoSelectedPadId && !pads.find(p => p.id === autoSelectedPadId)) {
        autoSelectedPadId = null; // Instantly fall back if current pad disconnected
      }
      for (const p of pads) {
        const hasActivity = p.axes.some(a => Math.abs(a) > 0.1) || p.buttons.some(b => b.pressed);
        if (hasActivity) {
          autoSelectedPadId = p.id;
          break; // First one with activity wins
        }
      }
    }

    let activeIdStr = activeGamepadId === null ? (autoSelectedPadId || (pads[0] ? pads[0].id : null)) : activeGamepadId;

    if (showGamepadModal) {
      gamepads = pads.map((p) => {
        let joyconType = "Standard";
        let n = p.id;
        if (n.includes("Joy-Con (L)")) joyconType = "Single L";
        else if (n.includes("Joy-Con (R)")) joyconType = "Single R";
        else if (n.includes("Extended") || n.includes("Joy-Con")) joyconType = "Dual";
        
        let dname = n.split(' (Vendor')[0];
        if (joyconType === "Dual") dname = "Nintendo Joy-Cons (Merged)";

        return {
          id: p.id,
          name: dname,
          joyconType: joyconType,
          battery: "OS Managed",
          active: activeIdStr === p.id
        };
      });
    }

    const pad = pads.find(p => p.id === activeIdStr);
    if (!pad || activeGamepadId === 'none') {
      controllerStatus = "Not Detected";
      return;
    }

    // Strip out the ugly Vendor/Product ID string that Chrome appends
    let joyconType = "Standard";
    let n = pad.id;
    if (n.includes("Joy-Con (L)")) joyconType = "Single L";
    else if (n.includes("Joy-Con (R)")) joyconType = "Single R";
    else if (n.includes("Extended") || n.includes("Joy-Con")) joyconType = "Dual";
    
    let cleanName = n.split(' (Vendor')[0];
    if (joyconType === "Dual") cleanName = "Nintendo Joy-Cons (Merged)";

    controllerStatus = cleanName;
    controllerBattery = "OS Managed";
    controllerType = joyconType;

    const deadzone = 0.1;
    let lx = Math.abs(pad.axes[0]) < deadzone ? 0 : pad.axes[0];
    let ly = Math.abs(pad.axes[1]) < deadzone ? 0 : pad.axes[1];
    let rx = Math.abs(pad.axes[2]) < deadzone ? 0 : pad.axes[2];
    let ry = Math.abs(pad.axes[3]) < deadzone ? 0 : pad.axes[3];

    let up = pad.buttons[12]?.pressed || false;
    let down = pad.buttons[13]?.pressed || false;
    let left = pad.buttons[14]?.pressed || false;
    let right = pad.buttons[15]?.pressed || false;
    const r1 = pad.buttons[5]?.pressed || false; // R1

    let deadman = false;
    let aiToggle = false;
    let syncToggle = false;
    let menuSettings = false;
    let menuAi = false;

    let btnPlus = pad.buttons[9]?.pressed || false; // Start/Options/Plus
    let btnMinus = pad.buttons[8]?.pressed || false; // Select/Share/Minus
    let btnHome = pad.buttons[16]?.pressed || false; // Home
    let btnCapture = pad.buttons[17]?.pressed || false; // Capture
    
    let btnNorth = pad.buttons[3]?.pressed || false; // Y/X
    let btnSouth = pad.buttons[0]?.pressed || false; // B/A
    let btnEast = pad.buttons[1]?.pressed || false; // A/B
    let btnWest = pad.buttons[2]?.pressed || false; // X/Y
    let btnL1 = pad.buttons[4]?.pressed || false; // L1 / SL
    let btnR1 = pad.buttons[5]?.pressed || false; // R1 / SR
    let btnL2 = pad.buttons[6]?.pressed || false; // L2 / L / R (shoulder)
    let btnR2 = pad.buttons[7]?.pressed || false; // R2 / ZL / ZR (trigger)

    // If it's a Single Joy-Con, fix its layout
    const isSingleJoyCon = pad.id.includes("Joy-Con (L)") || pad.id.includes("Joy-Con (R)");
    if (isSingleJoyCon) {
      if (lx === 0 && ly === 0) {
        if (pad.buttons[12]?.pressed) ly = -1;
        if (pad.buttons[13]?.pressed) ly = 1; 
        if (pad.buttons[14]?.pressed) lx = -1;
        if (pad.buttons[15]?.pressed) lx = 1; 
      } else if (Math.abs(rx) > 0 || Math.abs(ry) > 0) {
        lx = rx;
        ly = ry;
      }
      
      if (!btnL1 && !btnR1) {
        up = btnNorth;
        down = btnSouth;
        left = btnWest;
        right = btnEast;
      }

      aiToggle = btnL2;
      syncToggle = btnR2;
      deadman = btnL1 || btnR1; // SL or SR
      
      menuSettings = btnPlus || btnMinus;
      menuAi = btnHome || btnCapture;
    } else {
      aiToggle = btnNorth;
      syncToggle = btnWest;
      deadman = btnL2 || btnR2; // L2 or R2
      
      menuSettings = btnPlus;
      menuAi = btnMinus;
    }

    // UI Menu Toggles
    if (menuSettings && !(lastWebGamepadState.btnPlus || lastWebGamepadState.btnMinus)) settingsOpen = !settingsOpen;
    if (menuAi && !(lastWebGamepadState.btnHome || lastWebGamepadState.btnCapture)) aiControlsMinimized = !aiControlsMinimized;

    // AI & Robot Control Toggles
    // Prevent toggling if standard gamepad and L1/R1 are held (avoid conflict with speed combos)
    if (isSingleJoyCon || (!btnL1 && !btnR1)) {
        if (aiToggle && !lastWebGamepadState.aiToggle) isAutonomous = !isAutonomous;
        if (syncToggle && !lastWebGamepadState.syncToggle) viewportTurn = !viewportTurn;
    }

    // Speed Adjustments (SL/SR + East/South)
    let speedChanged = false;
    if (btnL1) {
        if (btnEast && !lastWebGamepadState.btnEast) { motorSpeed = Math.min(100, motorSpeed + 5); speedChanged = true; }
        if (btnSouth && !lastWebGamepadState.btnSouth) { motorSpeed = Math.max(0, motorSpeed - 5); speedChanged = true; }
    }
    if (btnR1) {
        if (btnEast && !lastWebGamepadState.btnEast) { servoSensitivity = Math.min(100, servoSensitivity + 5); speedChanged = true; }
        if (btnSouth && !lastWebGamepadState.btnSouth) { servoSensitivity = Math.max(0, servoSensitivity - 5); speedChanged = true; }
    }
    
    if (speedChanged) {
        invoke('update_settings', { motorSpeed, servoSensitivity }).catch(console.error);
    }

    if (deadman !== lastWebGamepadState.deadman) {
        deadmanActive = deadman;
    }

    joystickX = lx;
    joystickY = ly;
    cameraX = rx;
    cameraY = ry;

    gpDpadUp = up;
    gpDpadDown = down;
    gpDpadLeft = left;
    gpDpadRight = right;
    // Only snap if R1 is pressed without E/S combos
    gpBtnSnap = btnR1 && !btnEast && !btnSouth && !isSingleJoyCon;

    const changed = 
      lx !== lastWebGamepadState.lx || ly !== lastWebGamepadState.ly || 
      rx !== lastWebGamepadState.rx || ry !== lastWebGamepadState.ry ||
      up !== lastWebGamepadState.up || down !== lastWebGamepadState.down ||
      left !== lastWebGamepadState.left || right !== lastWebGamepadState.right ||
      gpBtnSnap !== lastWebGamepadState.r1 || speedChanged ||
      deadman !== lastWebGamepadState.deadman ||
      btnPlus !== lastWebGamepadState.btnPlus || btnMinus !== lastWebGamepadState.btnMinus ||
      btnHome !== lastWebGamepadState.btnHome || btnCapture !== lastWebGamepadState.btnCapture ||
      btnNorth !== lastWebGamepadState.btnNorth || btnWest !== lastWebGamepadState.btnWest ||
      aiToggle !== lastWebGamepadState.aiToggle || syncToggle !== lastWebGamepadState.syncToggle;

    if (changed) {
      sendKeyboardCommand(true, true);
      lastWebGamepadState = { 
        lx, ly, rx, ry, up, down, left, right, 
        r1: gpBtnSnap, deadman, 
        btnNorth, btnSouth, btnEast, btnWest, 
        btnPlus, btnMinus, btnHome, btnCapture,
        aiToggle, syncToggle
      };
    }
  }

  function runCameraLoop(timestamp: number) {
    const dt = timestamp - lastFrameTime;
    lastFrameTime = timestamp;

    pollWebGamepad();

    if (connected && servoMode === 'incremental' && (cameraX !== 0 || cameraY !== 0 || centerCamera)) {
      if (centerCamera) {
        pan = 0.0;
        tilt = 0.0;
        centerCamera = false;
      } else {
        const speed = (Number(servoSensitivity) / 100.0) * 0.0015; // 1.5 units per second at max speed
        pan += cameraX * speed * dt;
        tilt += -cameraY * speed * dt;
        pan = Math.max(-1, Math.min(1, pan));
        tilt = Math.max(-1, Math.min(1, tilt));
      }
      sendKeyboardCommand(true);
    }
    animationFrameId = requestAnimationFrame(runCameraLoop);
  }

  onMount(async () => {
    // Hide initial loader gracefully
    const loader = document.getElementById('initial-loader');
    if (loader) {
      loader.style.opacity = '0';
      setTimeout(() => loader.remove(), 500);
    }

    const savedIp = localStorage.getItem('picar-ip');
    if (savedIp) ipAddress = savedIp;
    
    lastFrameTime = performance.now();
    animationFrameId = requestAnimationFrame(runCameraLoop);
    unlistens.push(await listen('ws-connected', () => { connected = true; isDisconnecting = false; }));
    unlistens.push(await listen('ws-disconnected', () => { connected = false; isDisconnecting = false; }));
    unlistens.push(await listen('telemetry', (event: any) => {
      if (isDisconnecting) return;
      connected = true; // Recover connection state if UI hot-reloads
      
      distance = event.payload.distance?.toFixed(1) || "--";
      light = event.payload.left_light?.toFixed(1) || "--";
      
      if (event.payload.boxes) {
        boundingBoxes = event.payload.boxes;
      } else if (!event.payload.boxes && !isAutonomous) {
        // If AI is off, clear bounding boxes. If AI is on but no boxes in telemetry, 
        // we might be getting them from json-message broadcast instead!
        boundingBoxes = [];
      }
      
      if (event.payload.error) {
        connectionError = event.payload.error;
      } else {
        connectionError = '';
      }
      
      let debugParts = [];
      if (event.payload.ai_off) {
        aiDebugInfo = "";
      } else {
        if (event.payload.shape) debugParts.push(`Shape: ${event.payload.shape}`);
        if (event.payload.max_conf !== undefined) debugParts.push(`MaxConf: ${event.payload.max_conf.toFixed(3)}`);
        debugParts.push(`Boxes: ${boundingBoxes.length}`);
      }
      // Only set aiDebugInfo if we have something to say, or clear it if ai_off
      if (debugParts.length > 0) aiDebugInfo = debugParts.join(' | ');
      
      if (event.payload.frame) {
        videoBlobUrl = "data:image/jpeg;base64," + event.payload.frame;
      }
    }));

    unlistens.push(await listen('json-message', (event: any) => {
      if (event.payload && event.payload.type === 'bounding_boxes') {
        boundingBoxes = event.payload.boxes || [];
        // Update debug info for iOS client
        aiDebugInfo = `[Desktop Stream] Boxes: ${boundingBoxes.length}`;
      }
    }));
    
    // Setup keyboard listeners
    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('keyup', handleKeyup);
  });
  
  onDestroy(() => {
    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    unlistens.forEach(u => u());
    window.removeEventListener('keydown', handleKeydown);
    window.removeEventListener('keyup', handleKeyup);
  });
</script>


<main class="dashboard immersive">
  <!-- Fullscreen Video -->
  {#if connected}
    <div class="video-wrapper">

      {#if videoBlobUrl}
        <img class="fullscreen-video" src={videoBlobUrl} alt="Live MJPEG stream" />
        <!-- Bounding Boxes Overlay -->
        {#each boundingBoxes.filter(b => b.label === targetClass) as box}
          <div 
            class="bounding-box" 
            style="
              left: {(box.x / 320) * 100}%; 
              top: {(box.y / 240) * 100}%; 
              width: {(box.width / 320) * 100}%; 
              height: {(box.height / 240) * 100}%;
              border-color: #10b981;
            "
          >
            <div class="box-label" style="background-color: #10b981;">
              {box.label} {Math.round(box.confidence * 100)}% | {distance}
            </div>
          </div>
        {/each}
      {:else}
        <div class="stream-placeholder fullscreen-video center-text">Video Stream Active... Waiting for frames.</div>
      {/if}
    </div>
  {:else}
    <div class="stream-placeholder fullscreen-video center-text">Waiting for connection...</div>
  {/if}

  <!-- HUD Overlays -->
  <div class="hud-layer" style="opacity: {uiOpacity};">
    <!-- Top Bar -->
    <header class="hud-top safe-area-left safe-area-right safe-area-top">
      <div class="logo-area glass-panel">
        <h1>Picar-Vision</h1>
        <span class="badge" class:active={connected}>
          {connected ? 'Connected' : 'Disconnected'}
        </span>
      </div>
      <div class="top-controls" style="align-items: center;">
        <button class="btn-primary glass-panel icon-btn" onclick={() => showRobotControls = !showRobotControls}>
          🎮
        </button>
        <button class="btn-primary glass-panel icon-btn" onclick={() => showSettings = true}>
          ⚙️
        </button>
      </div>
    </header>

    <!-- Robot Controls Window -->
    {#if showRobotControls}
      <div class="hud-robot-controls glass-panel safe-area-right" style="position: absolute; right: 1rem; top: 5rem; padding: 1rem 1rem 1.25rem 1rem; width: 150px; z-index: 10; display: flex; flex-direction: column; gap: 0.75rem;">
        <h4 style="margin:0; font-size: 0.85rem; text-align: right; color: var(--accent-primary);">Robot Controls</h4>
        <label class="slider-group" style="font-size: 0.75rem; margin: 0; display: flex; flex-direction: column; align-items: flex-end; gap: 0.5rem;">
          <span>Motor: {motorSpeed}%</span>
          <input type="range" min="0" max="100" bind:value={motorSpeed} oninput={() => sendKeyboardCommand()} style="margin: 0; width: 100%; height: 4px;" />
        </label>
        <label class="slider-group" style="font-size: 0.75rem; margin: 0; display: flex; flex-direction: column; align-items: flex-end; gap: 0.5rem;">
          <span>Servo: {servoSensitivity}%</span>
          <input type="range" min="0" max="100" bind:value={servoSensitivity} oninput={() => sendKeyboardCommand()} style="margin: 0; width: 100%; height: 4px;" />
        </label>
      </div>
    {/if}

    <!-- Bottom Controls -->
    <div class="hud-controls safe-area-bottom safe-area-left safe-area-right">
      <!-- Joysticks (Only if no gamepad) -->
      {#if !isGamepadConnected}
        <div class="joystick-wrapper">
          <div class="joystick-base" onpointerdown={(e) => handlePointerDown(e, 'L')} onpointermove={handlePointerMove} onpointerup={handlePointerUp} onpointercancel={handlePointerUp} style="cursor: crosshair; touch-action: none;">
            <div class="joystick-stick" style="transform: translate({joystickX * 30}px, {joystickY * 30}px)"></div>
          </div>
        </div>
      {:else}
        <div class="joystick-spacer" style="width:120px;height:120px;"></div>
      {/if}
      
      <div class="ai-controls glass-panel" style="display: flex; flex-direction: column; gap: 0.25rem; padding: 1.5rem 1rem 0.5rem 1rem; border-radius: 16px; align-items: center; background: rgba(10, 10, 15, 0.75); border: 1px solid rgba(255,255,255,0.1); width: fit-content; margin: 0 auto; z-index: 10; position: relative; transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1); transform: translateY({aiControlsMinimized ? 'calc(100% - 40px)' : '0'});">
        <!-- Swipe / Drag Handle -->
        <div 
          onpointerdown={handleSwipeStart} 
          onpointerup={handleSwipeEnd}
          style="width: 100%; height: 32px; position: absolute; top: 0; left: 0; display: flex; justify-content: center; align-items: center; cursor: grab; z-index: 11;"
        >
          <div style="width: 48px; height: 6px; background: rgba(255, 255, 255, 0.6); border-radius: 3px;"></div>
        </div>

        <div style="font-size: 0.7rem; font-weight: 700; color: var(--accent-primary); letter-spacing: 1px; text-transform: uppercase;">
          {isAutonomous ? 'Auto-Track' : 'Manual Mode'}
        </div>
        
        {#if isAutonomous}
          <!-- iOS Camera Style Slide Wheel -->
          <div 
            class="slide-wheel"
            bind:this={wheelContainer}
            onscroll={handleWheelScroll}
            style="display: flex; gap: 1.5rem; align-items: center; margin: 0.25rem 0; overflow-x: auto; scroll-snap-type: x mandatory; width: 180px; height: 35px; scrollbar-width: none; -ms-overflow-style: none; -webkit-overflow-scrolling: touch; mask-image: linear-gradient(to right, transparent, black 30%, black 70%, transparent); position: relative;"
          >
            <!-- Spacer -->
            <div style="min-width: calc(50% - 10px); flex-shrink: 0;"></div>
            
            {#each trackingTargets as target (target.id)}
              <div 
                data-target={target.id}
                style="scroll-snap-align: center; font-size: {targetClass === target.id ? '0.9rem' : '0.75rem'}; font-weight: {targetClass === target.id ? '700' : '500'}; color: {targetClass === target.id ? 'var(--accent-primary)' : 'var(--text-secondary)'}; text-transform: uppercase; cursor: pointer; transition: all 0.2s; white-space: nowrap; flex-shrink: 0; padding: 0 0.5rem;"
                onclick={() => { targetClass = target.id; scrollToTarget(target.id); }}
              >
                {target.label}
              </div>
            {/each}
            
            <!-- Spacer -->
            <div style="min-width: calc(50% - 10px); flex-shrink: 0;"></div>
          </div>
        {:else}
          <!-- Manual Sensors & Accessories -->
          <div style="display: flex; gap: 0.75rem; align-items: center; margin: 0.15rem 0; width: 100%; justify-content: center;">
            <div style="display: flex; flex-direction: column; align-items: center; font-size: 0.65rem; color: var(--text-secondary);">
              Dist
              <span style="color: white; font-weight: bold; font-size: 0.8rem;">{distance}</span>
            </div>
            <div class="divider" style="width: 1px; height: 20px; background: rgba(255,255,255,0.2);"></div>
            <div style="display: flex; flex-direction: column; align-items: center; font-size: 0.65rem; color: var(--text-secondary);">
              Light
              <span style="color: white; font-weight: bold; font-size: 0.8rem;">{light}</span>
            </div>
            <div class="divider" style="width: 1px; height: 20px; background: rgba(255,255,255,0.2);"></div>
            <button class="v-key btn-led" class:active={ledActive} onpointerdown={(e) => { ledActive = true; sendKeyboardCommand(); e.preventDefault(); }} onpointerup={(e) => { ledActive = false; sendKeyboardCommand(); e.preventDefault(); }} style="height: 28px; min-width: 40px; font-size: 0.7rem; padding: 0 0.5rem;">LED</button>
            <button class="v-key btn-buzz" class:active={buzzerActive} onpointerdown={(e) => { buzzerActive = true; sendKeyboardCommand(); e.preventDefault(); }} onpointerup={(e) => { buzzerActive = false; sendKeyboardCommand(); e.preventDefault(); }} style="height: 28px; min-width: 40px; font-size: 0.7rem; padding: 0 0.5rem;">Buzz</button>
          </div>
        {/if}
        
        <div style="display: flex; gap: 1rem; align-items: center; width: 100%; justify-content: center; margin-top: 0.25rem; padding: 0 0.5rem;">
          <!-- Mode Toggle -->
          <div style="display: flex; flex-direction: column; align-items: center; gap: 2px;">
            <span style="font-size:0.55rem; color:var(--text-secondary); text-transform:uppercase; font-weight: bold;">AI Mode</span>
            <label class="switch" style="transform: scale(0.65); margin: 0;">
              <input type="checkbox" bind:checked={isAutonomous} />
              <span class="slider"></span>
            </label>
          </div>

          <!-- Play/Pause Deadman -->
          <button class="icon-btn" style="width: 44px; height: 44px; border-radius: 50%; font-size: 1.2rem; background: {deadmanActive ? 'var(--accent-primary)' : 'rgba(255,255,255,0.1)'}; color: {deadmanActive ? '#000' : '#fff'}; display: flex; align-items: center; justify-content: center; border: 1px solid rgba(255,255,255,0.2); transition: all 0.2s; touch-action: none; user-select: none;" onpointerdown={(e) => { deadmanActive = true; e.preventDefault(); }} onpointerup={(e) => { deadmanActive = false; e.preventDefault(); }} onpointercancel={(e) => { deadmanActive = false; e.preventDefault(); }} oncontextmenu={(e) => e.preventDefault()}>
            {deadmanActive ? '⏸' : '▶'}
          </button>

          <!-- Follow / Roam Toggle -->
          <div style="display: flex; flex-direction: column; align-items: center; gap: 2px;">
            <span style="font-size:0.55rem; color:var(--text-secondary); text-transform:uppercase; font-weight: bold;">
              {isAutonomous ? 'Auto Drive' : 'Sync Steer'}
            </span>
            <label class="switch" style="transform: scale(0.65); margin: 0;">
              <input type="checkbox" bind:checked={viewportTurn} onchange={() => sendKeyboardCommand()} />
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </div>

      {#if !isGamepadConnected}
        <div class="joystick-wrapper">
          <div class="joystick-base" onpointerdown={(e) => handlePointerDown(e, 'R')} onpointermove={handlePointerMove} onpointerup={handlePointerUp} onpointercancel={handlePointerUp} style="cursor: crosshair; touch-action: none;">
            <div class="joystick-stick" style="transform: translate({cameraX * 30}px, {cameraY * 30}px)"></div>
          </div>
        </div>
      {:else}
        <div class="joystick-spacer" style="width:120px;height:120px;"></div>
      {/if}
    </div>
  </div>
</main>

<!-- Settings Sidebar -->
{#if showSettings}
<div class="modal-backdrop" onclick={() => showSettings = false}>
  <div class="settings-sidebar glass-panel safe-area-right safe-area-top safe-area-bottom" onclick={(e) => e.stopPropagation()}>
    <div class="sidebar-header">
      <h2>Settings</h2>
      <div style="display: flex; gap: 0.75rem; align-items: center; margin-right: 0.25rem;">
        <button class="icon-btn" style="background: rgba(255,255,255,0.1); border-radius: 50%; transition: transform 0.3s ease; transform: rotate({allExpanded ? '180deg' : '0deg'}); width: 44px; height: 44px; display: flex; align-items: center; justify-content: center; font-size: 1.2rem;" onclick={() => toggleAllSettings(!allExpanded)}>
          ▼
        </button>
        <button class="btn-danger icon-btn" style="width: 44px; height: 44px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 1.5rem;" onclick={() => showSettings = false}>×</button>
      </div>
    </div>
    
    <div class="settings-content">

      <!-- 1. Connection -->
      <div class="settings-group">
        <h3 onclick={() => expandedSections.connection = !expandedSections.connection} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; margin-bottom: {expandedSections.connection ? '1rem' : '0'}; transition: margin 0.2s;">
          Connection
          <span style="font-size: 0.8rem; color: var(--text-secondary);">{expandedSections.connection ? '▼' : '▶'}</span>
        </h3>
        {#if expandedSections.connection}
          <div>
            <input type="text" bind:value={ipAddress} placeholder="Raspberry Pi IP" disabled={connected} onkeydown={(e) => e.key === 'Enter' && !connected && toggleConnection()} />
            <button onclick={toggleConnection} class={connected ? 'btn-danger' : 'btn-primary'} style="width: 100%; margin-top: 0.5rem;">
              {connected ? 'Disconnect' : 'Connect'}
            </button>
            {#if connectionError}
              <div style="color: red; margin-top: 0.5rem; font-size: 0.9em; text-align: center;">
                {connectionError}
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- 2. Controller -->
      <div class="settings-group">
        <h3 onclick={() => expandedSections.controller = !expandedSections.controller} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; margin-bottom: {expandedSections.controller ? '1rem' : '0'}; transition: margin 0.2s;">
          Controller
          <span style="font-size: 0.8rem; color: var(--text-secondary);">{expandedSections.controller ? '▼' : '▶'}</span>
        </h3>
        {#if expandedSections.controller}
          <div>
            <div class="sensor-row">
              <span>Status:</span>
              <span class="value" class:disconnected={controllerStatus === "Not Detected"}>{controllerStatus !== "Not Detected" ? controllerType : "Not Detected"}</span>
            </div>
            {#if controllerStatus !== "Not Detected"}
              <div class="sensor-row">
                <span>Battery:</span>
                <span class="value">{controllerBattery}</span>
              </div>
            {/if}
            <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
              <button class="btn-primary" onclick={() => showGamepadModal = true} style="flex: 1;">Find</button>
              {#if controllerStatus !== "Not Detected"}
                <button class="btn-primary" onclick={() => showCalibration = true} style="flex: 1;">Calibrate</button>
              {/if}
            </div>
          </div>
        {/if}
      </div>

      <!-- 3. UI Preferences -->
      <div class="settings-group">
        <h3 onclick={() => expandedSections.uiPreferences = !expandedSections.uiPreferences} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; margin-bottom: {expandedSections.uiPreferences ? '1rem' : '0'}; transition: margin 0.2s;">
          UI Preferences
          <span style="font-size: 0.8rem; color: var(--text-secondary);">{expandedSections.uiPreferences ? '▼' : '▶'}</span>
        </h3>
        {#if expandedSections.uiPreferences}
          <div>
            <label class="slider-group">
              UI Opacity: {Math.round(uiOpacity * 100)}%
              <input type="range" min="0.1" max="1" step="0.05" bind:value={uiOpacity} />
            </label>
          </div>
        {/if}
      </div>

      <!-- 4. AI Tracking Targets -->
      <div class="settings-group">
        <h3 onclick={() => expandedSections.aiTracking = !expandedSections.aiTracking} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; margin-bottom: {expandedSections.aiTracking ? '1rem' : '0'}; transition: margin 0.2s;">
          AI Tracking Targets
          <span style="font-size: 0.8rem; color: var(--text-secondary);">{expandedSections.aiTracking ? '▼' : '▶'}</span>
        </h3>
        {#if expandedSections.aiTracking}
          <div>
            <p style="font-size: 0.75rem; color: var(--text-secondary); margin-top: 0; margin-bottom: 0.5rem;">Configure which targets appear in the slide wheel.</p>
            <div style="display: flex; flex-direction: column; gap: 0.5rem; max-height: 200px; overflow-y: auto; padding-right: 0.5rem;">
              {#each allTrackingTargets as target, index (target.id)}
                <div 
                  draggable="true"
                  ondragstart={(e) => handleDragStart(e, index)}
                  ondragover={handleDragOver}
                  ondrop={(e) => handleDrop(e, index)}
                  ondragend={() => draggedIndex = null}
                  style="display: flex; justify-content: space-between; align-items: center; background: rgba(255,255,255,0.05); padding: 0.25rem 0.5rem; border-radius: 6px; cursor: grab; opacity: {draggedIndex === index ? 0.5 : 1};"
                >
                  <div style="display: flex; align-items: center; gap: 0.75rem;">
                    <div style="color: rgba(255,255,255,0.3); font-size: 1.2rem; user-select: none;">≡</div>
                    <span style="font-size:0.85rem; color:var(--text-secondary); text-transform: capitalize;">{target.label}</span>
                  </div>
                  <label class="switch" style="transform: scale(0.8); margin: 0;">
                    <input type="checkbox" bind:checked={target.active} />
                    <span class="slider"></span>
                  </label>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- 5. AI Debug -->
      <div class="settings-group">
        <h3 onclick={() => expandedSections.aiDebug = !expandedSections.aiDebug} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; margin-bottom: {expandedSections.aiDebug ? '1rem' : '0'}; transition: margin 0.2s;">
          AI Debug
          <span style="font-size: 0.8rem; color: var(--text-secondary);">{expandedSections.aiDebug ? '▼' : '▶'}</span>
        </h3>
        {#if expandedSections.aiDebug}
          <div>
            {#if connectionError}
              <div style="color: red; margin-bottom: 0.5rem; font-size: 0.9em; text-align: center;">
                {connectionError}
              </div>
            {/if}
            <pre class="debug-text" style="white-space: pre-wrap; font-size: 0.75rem;">{aiDebugInfo || "Waiting for frames..."}</pre>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
{/if}

{#if showCalibration}
<div class="modal-backdrop" style="z-index: 2000;">
  <div class="modal-content glass-panel">
    <h2>Joystick Calibration</h2>
    <p style="font-size: 0.85rem; color: var(--text-secondary); margin-bottom: 1rem;">Values are in thousandths (e.g., 200 = 0.20)</p>
    
    <div class="cal-grid">
      <div>
        <h4>Left Stick</h4>
        <label>X Center: <input type="number" bind:value={calLxCenter} /></label>
        <label>Y Center: <input type="number" bind:value={calLyCenter} /></label>
        <label>X Deadzone: <input type="number" bind:value={calLxDeadzone} /></label>
        <label>Y Deadzone: <input type="number" bind:value={calLyDeadzone} /></label>
      </div>
      <div>
        <h4>Right Stick</h4>
        <label>X Center: <input type="number" bind:value={calRxCenter} /></label>
        <label>Y Center: <input type="number" bind:value={calRyCenter} /></label>
        <label>X Deadzone: <input type="number" bind:value={calRxDeadzone} /></label>
        <label>Y Deadzone: <input type="number" bind:value={calRyDeadzone} /></label>
      </div>
    </div>

    <div class="modal-actions">
      <button class="btn-primary" onclick={saveCalibration}>Save</button>
      <button class="btn-danger" onclick={() => showCalibration = false}>Cancel</button>
    </div>
  </div>
</div>
{/if}

{#if showGamepadModal}
<div class="modal-backdrop" style="z-index: 2000;">
  <div class="modal-content glass-panel" style="min-width: 500px;">
    <div style="display: flex; justify-content: space-between; align-items: center;">
      <h2>Connected Gamepads</h2>
      <div style="display: flex; align-items: center; gap: 0.75rem;">
        <span style="font-size: 0.9rem; font-weight: bold; color: {activeGamepadId === null ? 'var(--primary-color)' : 'var(--text-secondary)'};">Auto Select</span>
        <label class="switch">
          <input type="checkbox" checked={activeGamepadId === null} onchange={(e) => toggleAutoSelect((e.target as HTMLInputElement).checked)} />
          <span class="slider"></span>
        </label>
      </div>
    </div>
    
    {#if gamepads.length === 0}
      <p style="color: var(--text-secondary); text-align: center; padding: 2rem 0;">No gamepads detected by the system.</p>
    {:else}
      <div class="gamepads-list">
        {#each gamepads as gp}
          <div class="gamepad-item" class:active={gp.active}>
            <div class="gamepad-info">
              <strong>{gp.name}</strong> ({gp.joyconType})
              <span class="battery-badge">{gp.battery}</span>
            </div>
            <div class="gamepad-actions">
              <button class="btn-primary" disabled={gp.active} onclick={() => setGamepadMode(gp.id)}>Select</button>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <div class="modal-actions" style="justify-content: space-between;">
      <button class="btn-danger" onclick={() => setGamepadMode('none')}>Disconnect All</button>
      <button class="btn-primary" onclick={() => showGamepadModal = false}>Close</button>
    </div>
  </div>
</div>
{/if}

<style>

  /* IMMERSIVE LAYOUT */
  .immersive {
    position: relative;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: #000;
    padding: 0;
    display: block;
  }
  .fullscreen-video {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    z-index: 1;
  }
  
  .video-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
  }
  
  .bounding-box {
    position: absolute;
    border: 3px solid #10b981;
    z-index: 2;
    pointer-events: none;
    box-sizing: border-box;
    box-shadow: 0 0 10px rgba(0,0,0,0.5);
  }
  
  .box-label {
    position: absolute;
    top: -24px;
    left: -3px;
    background-color: #10b981;
    color: white;
    font-size: 0.75rem;
    font-weight: bold;
    padding: 2px 6px;
    border-radius: 4px 4px 0 0;
    white-space: nowrap;
    text-transform: capitalize;
  }
  
  .center-text {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    font-weight: 500;
  }
  
  .hud-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 10;
    pointer-events: none; /* Let touches pass to video if needed */
    transition: opacity 0.2s ease;
  }
  .hud-layer > * {
    pointer-events: auto;
  }
  
  /* Safe Areas */
  .safe-area-top { padding-top: env(safe-area-inset-top, 10px); }
  .safe-area-bottom { padding-bottom: env(safe-area-inset-bottom, 20px); }
  .safe-area-left { padding-left: env(safe-area-inset-left, 20px); }
  .safe-area-right { padding-right: env(safe-area-inset-right, 20px); }

  .hud-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1.5rem;
  }
  
  .hud-top .top-controls {
    display: flex;
    gap: 0.75rem;
  }
  
  .hud-top .logo-area {
    padding: 0.5rem 1rem;
    border-radius: 12px;
  }

  .icon-btn {
    width: 44px;
    height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    border-radius: 12px;
    padding: 0;
  }

  .hud-telemetry {
    position: absolute;
    top: 50%;
    left: 1.5rem;
    transform: translateY(-50%);
    padding: 1rem;
    border-radius: 12px;
    min-width: 150px;
  }

  .hud-debug {
    position: absolute;
    bottom: 5.5rem;
    right: 1.5rem;
    padding: 1rem;
    border-radius: 12px;
    min-width: 200px;
    max-width: 350px;
    max-height: 40vh;
    overflow-y: auto;
  }
  
  .hud-debug h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.85rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 1px;
  }
  
  .debug-text {
    margin: 0 0 0.25rem 0;
    color: #10b981;
    font-family: monospace;
    font-size: 0.85rem;
    white-space: pre-wrap;
  }
  
  .debug-text.text-danger {
    color: #ef4444;
  }

  .hud-controls {
    position: absolute;
    bottom: 1.5rem;
    left: 0;
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    padding: 0 3rem;
    box-sizing: border-box;
  }
  
  /* Settings Sidebar */
  .settings-sidebar {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    width: 350px;
    max-width: 90vw;
    background: rgba(15, 23, 42, 0.95);
    backdrop-filter: blur(20px);
    border-left: 1px solid rgba(255, 255, 255, 0.1);
    z-index: 1000;
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
    box-sizing: border-box;
    transform: translateX(0);
    animation: slideIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }
  
  @keyframes slideIn {
    from { transform: translateX(100%); }
    to { transform: translateX(0); }
  }

  .sidebar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  .sidebar-header h2 { font-size: 1.25rem; color: #fff; margin: 0; }
  
  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    overflow-y: auto;
    flex: 1;
    padding-right: 0.5rem;
  }
  
  .settings-group {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .settings-group h3 {
    font-size: 0.9rem;
    color: var(--accent-primary);
    margin: 0 0 0.25rem 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* Make Joysticks Bigger */
  .joystick-base {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.1);
    border: 2px solid rgba(255, 255, 255, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    backdrop-filter: blur(10px);
  }
  .joystick-stick {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    background: var(--accent-primary);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.4), inset 0 2px 4px rgba(255, 255, 255, 0.4);
    transition: transform 0.05s ease-out;
  }

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
    gap: 1rem;
    font-size: clamp(0.75rem, 1.5vw, 1rem);
  }

  .sensor-row:last-child {
    margin-bottom: 0;
  }

  .sensor-row span:first-child {
    white-space: nowrap;
    flex-shrink: 0;
  }

  .value {
    color: var(--accent-primary);
    font-family: monospace;
    font-size: clamp(0.8rem, 2vw, 1.1rem);
    text-align: right;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
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

  .key-mode-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .toggle-container {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
  }

  .toggle-label {
    font-size: 0.75rem;
    color: var(--text-secondary);
    text-align: center;
    line-height: 1.2;
    transition: color 0.2s;
  }

  .toggle-label.active {
    color: var(--accent-primary);
    font-weight: 600;
  }

  .switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    transition: .4s;
    border-radius: 24px;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 16px;
    width: 16px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: .4s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: var(--accent-primary);
    border-color: var(--accent-primary);
  }

  input:checked + .slider:before {
    transform: translateX(20px);
  }

  input:disabled + .slider {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: #1e293b;
    padding: 2rem;
    border-radius: 12px;
    min-width: 400px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .cal-grid {
    display: flex;
    gap: 2rem;
  }

  .cal-grid h4 {
    margin-bottom: 0.5rem;
    color: var(--accent-primary);
  }

  .cal-grid label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.85rem;
    margin-bottom: 0.5rem;
    color: var(--text-secondary);
  }

  .cal-grid input {
    width: 80px;
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    color: white;
    padding: 0.25rem;
    border-radius: 4px;
    text-align: right;
  }
  
  .styled-select {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    color: white;
    padding: 0.5rem;
    border-radius: 6px;
    width: 100%;
    font-size: 0.9rem;
    outline: none;
  }
  .styled-select option {
    background: #1e293b;
    color: white;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    margin-top: 1rem;
  }

  .gamepads-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-height: 300px;
    overflow-y: auto;
  }

  .gamepad-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
  }

  .gamepad-item.active {
    background: rgba(96, 165, 250, 0.1);
    border-color: var(--accent-primary);
  }

  .gamepad-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .gamepad-info strong {
    color: white;
    font-size: 1rem;
  }

  .battery-badge {
    font-size: 0.75rem;
    background: rgba(16, 185, 129, 0.2);
    color: var(--accent-success);
    padding: 2px 6px;
    border-radius: 4px;
    display: inline-block;
    width: max-content;
  }

  .gamepad-actions {
    display: flex;
    gap: 0.5rem;
  }

  .gamepad-actions button {
    font-size: 0.8rem;
    padding: 0.4rem 0.75rem;
  }


</style>
