<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Horse } from "../types";
  import HorseCard from "./Horse-Card.svelte";
    import { Commands } from "$src/api/ipc";

  let name = $state("");
  let default_horse: Horse = {
    id: "",
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
      await invoke<Horse>(Commands.add_horse, { horse: { ...default_horse, name } });
      void get_horses();
    } catch (e) {
      alert(e);
    }
  }

  async function get_horses() {
    try {
      horses = await invoke<Horse[]>(Commands.list_all_horses);
    } catch (e) {
      alert(e);
    }
  }

  async function delete_horse(id: number) {
    try {
      let res = await invoke<boolean>(Commands.remove_horse, { id });
      if (res) {
        horses = horses.filter((horse) => horse.id !== id);
      }
    } catch (e) {
      alert(e);
    }
  }

  $effect(() => {
    void get_horses();
  });
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
    <HorseCard {horse} deleteHorse={delete_horse} />
  {/each}
</div>
