<script>
  import { invoke } from '@tauri-apps/api/tauri'

  async function new_window() {
    await invoke('create_window')
  }
</script>

<div>
  <button on:click={new_window} class="bg-gray-200 p-2 rounded-md">Create Window</button>
</div>
