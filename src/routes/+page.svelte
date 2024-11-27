<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { getUser } from "./auth.svelte";
  import { Commands } from "$src/api/ipc";
  import type { Horse } from "$src/types";

  let { current: user } = getUser();
  let path = $state("");
  let horses: Horse[] = $state([]);

  function handleOpen() {
  }

  $effect(() => {
    if (!user) {
      goto("/login");
    }

    async function get_horses() {
      try {
        horses = await invoke<Horse[]>(Commands.list_all_horses);
        console.log(horses);
      } catch (e) {
        alert(e);
      }
    }
    get_horses();

  });
</script>

<div class="container mx-auto py-16">
  <h1 class="text-4xl text-center pb-2 text-primary">Welcome</h1>
  <h2 class="text-3xl text-center pb-10 text-secondary italic">
    Built by Tauri!(Rust BTW)
  </h2>

  <input
    type="text"
    placeholder="Open folder"
    class="w-full p-2 text-primary border-2 border-primary rounded-md"
    bind:value={path}
  />
  <button class="w-full p-2 text-primary border-2 border-primary rounded-md" onclick={handleOpen}
    >Open</button
  >
</div>
