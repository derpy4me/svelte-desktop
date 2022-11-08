<script>
  import { invoke } from '@tauri-apps/api/tauri'

  let name = ''
  let greetMsg = ''

  async function greet() {
    greetMsg = await invoke('greet', { name })
  }
</script>

<div>
  <input id="greet-input" placeholder="Enter a name..." bind:value={name} class="m-2 p-2 bg-gray-200 rounded-md" />
  <button on:click={greet} class="bg-gray-200 p-2 rounded-md">Greet</button>
  <p class="m-2 font-bold text-xl">{greetMsg}</p>
</div>
