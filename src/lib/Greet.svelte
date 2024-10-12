<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Horse } from "../types";

  let name = $state("");
  let default_horse: Horse = {
    id: 0,
    name: "Jimmy",
    breed: "English",
    color: "Brown",
    nationality: "English",
    age: 10,
    gender: "Male",
    weight: 200,
    height: 140,
    length: 250,
  };
  let horses: Horse[] = $state([]);

  async function add_horse() {
    try {
      let res = await invoke<Horse>("add_horse", { horse:{...default_horse, name} });

      if (res) {
        horses = [...horses, res];
      }
    } catch (e) {
      alert(e);
    }
  }
</script>

<div class="py-2 w-fit mx-auto my-4 grid rounded gap-4 px-4">
  <input
    id="greet-input"
    placeholder="Enter a name for horse..."
    bind:value={name}
    class="p-2 placeholder:text-primary placeholder:italic rounded"
  />
  <button
    onclick={add_horse}
    class="bg-primary p-2 rounded font-bold disabled:bg-primary/70 disabled:cursor-not-allowed"
    disabled={false}>Add</button
  >
</div>
<div class="grid grid-cols-4 gap-5">
  {#each horses as horse}
    <div class="grid gap-2">
      {#each Object.entries(horse ?? {}) as [key, v]}
        <p
          class="flex gap-2 border-2 p-2 rounded bg-primary border-black items-center max-w-full overflow-hidden"
        >
          <span class=" font-semibold">{key}:</span><span
            class="font-bold text-lg px-1 text-white truncate">{v}</span
          >
        </p>
      {/each}
    </div>
  {/each}
</div>
