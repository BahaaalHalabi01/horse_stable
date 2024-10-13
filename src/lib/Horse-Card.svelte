<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { EventHandler } from "svelte/elements";
  import type { Horse } from "../types";

  let { horse }: { horse: Horse } = $props();

  let delete_horse: EventHandler<MouseEvent, HTMLButtonElement> =
    async function (e) {
      let id = Number(e.currentTarget.value);
      try {
        await invoke<Horse>("delete_horse", { id });
      } catch (e) {
        alert(e);
      }
    };
</script>

<div class="grid gap-2 grid-cols-2 bg-primary text-secondary p-2 rounded">
  <p class="">{horse.name}</p>
  <button
    class="bg-primary rounded font-bold disabled:bg-primary/70 disabled:cursor-not-allowed w-fit p-1"
    disabled={false}
    value={horse.id.toString()}
    onclick={delete_horse}>X</button
  >
  <p class="">{horse.age}</p>
  <p class="">{horse.color}</p>
  <p class="">{horse.breed}</p>
  <p class="">{horse.gender}</p>
  <p class="">{horse.height}</p>
  <p class="">{horse.weight}</p>
  <p class="">{horse.nationality}</p>
</div>
