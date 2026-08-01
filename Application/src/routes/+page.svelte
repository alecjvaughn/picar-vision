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

  async function toggleConnection() {
    if (!connected) {
      try {
        await invoke('connect_to_pi', { ip: ipAddress });
      } catch (e) {
        console.error(e);
        alert(e);
      }
    } else {
      connected = false;
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
      light = event.payload.light?.toString() || "--";
    }));
    unlistens.push(await listen('video-frame', (event: any) => {
      const blob = new Blob([new Uint8Array(event.payload)], { type: 'image/jpeg' });
      if (videoBlobUrl) URL.revokeObjectURL(videoBlobUrl);
      videoBlobUrl = URL.createObjectURL(blob);
    }));
  });
  
  onDestroy(() => {
    unlistens.forEach(u => u());
    if (videoBlobUrl) URL.revokeObjectURL(videoBlobUrl);
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
</style>
