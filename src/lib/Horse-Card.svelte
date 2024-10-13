<script lang="ts">
  import type { EventHandler } from "svelte/elements";
  import type { Horse } from "../types";

  let {
    horse,
    deleteHorse,
  }: { horse: Horse; deleteHorse: (v: number) => void } = $props();

  let delete_horse: EventHandler<MouseEvent, HTMLButtonElement> = function (e) {
    deleteHorse(Number(e.currentTarget.value));
  };
</script>

<div class="grid gap-2 grid-cols-2 bg-primary text-secondary p-2 rounded">
  <p class="">{horse.name}</p>
  <button
    class="bg-red-700 rounded font-bold text-white w-fit p-2 text-xs ml-auto"
    disabled={false}
    value={horse.id.toString()}
    onclick={delete_horse}
  >
    X
  </button>
  <p class="">{horse.age}</p>
  <p class="">{horse.color}</p>
  <p class="">{horse.breed}</p>
  <p class="">{horse.gender}</p>
  <p class="">{horse.height}</p>
  <p class="">{horse.weight}</p>
  <p class="">{horse.nationality}</p>
</div>
