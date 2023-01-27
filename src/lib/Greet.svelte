<script lang="ts">
  import { json } from "@sveltejs/kit";
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let x = await fetch("http://localhost:8800/hello", {
      headers: {
        "Accept": "application/json",
        "Content-Type": 'application/json'
      },
      method: "POST",
      body: JSON.stringify({name})
    });

    greetMsg = x.json()
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}> Greet </button>
  </div>
  {#if greetMsg}
    {#await greetMsg then g }
      <p>{g.message}</p>  
    {/await}
  {/if}
</div>
