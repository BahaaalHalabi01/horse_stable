<script lang="ts">
  import type { EventHandler } from "svelte/elements";
  import type { Horse } from "../types";
  import { invoke } from "@tauri-apps/api/core";

  let {
    horse,
    deleteHorse,
  }: { horse: Horse; deleteHorse: (v: number) => void } = $props();

  let editMode = $state(false);

  let delete_horse: EventHandler<MouseEvent, HTMLButtonElement> = function (e) {
    deleteHorse(Number(e.currentTarget.value));
  };
  let handle_edit: EventHandler<MouseEvent, HTMLButtonElement> =
    async function (e) {
      try {
        await invoke<boolean>("update_horse", { horse });
      } catch (e) {
        alert(e);
      }
    };

  // let handle_change: EventHandler<Event, HTMLInputElement> = function (e) {
  //   let target = e.currentTarget.name as keyof Horse;
  //   let value = e.currentTarget.value as any;
  //
  //   if (target === "gender") {
  //     value = value === "Male" ? "Male" : "Female";
  //   }
  //
  //   horse[target] = value;
  // };
</script>

<div>
  <label>
    edit
    <input
      type="checkbox"
      checked={editMode}
      onchange={() => (editMode = !editMode)}
    />
  </label>
  <button onclick={handle_edit}> confirm </button>

  <div class="grid gap-2 grid-cols-2 bg-primary text-secondary p-2 rounded">
    <p class="truncate">{horse.name}</p>
    <button
      class="bg-red-700 rounded font-bold text-white w-fit p-2 text-xs ml-auto"
      disabled={false}
      value={horse.id.toString()}
      onclick={delete_horse}
    >
      X
    </button>

    <input value={horse.id.toString()} disabled={true} />
    <input bind:value={horse.name} disabled={!editMode} name="name" />
    <input bind:value={horse.breed} disabled={!editMode} name="breed" />
    <input bind:value={horse.color} disabled={!editMode} name="color" />
    <input
      bind:value={horse.nationality}
      disabled={!editMode}
      name="nationality"
    />
    <input
      bind:value={horse.age}
      disabled={!editMode}
      type="number"
      name="age"
    />
    <input bind:value={horse.gender} disabled={!editMode} name="gender" />
    <input
      bind:value={horse.weight}
      type="number"
      disabled={!editMode}
      name="weight"
    />
    <input
      bind:value={horse.height}
      disabled={!editMode}
      name="height"
      type="number"
    />
    <input
      bind:value={horse.length}
      disabled={!editMode}
      type="number"
      name="length"
    />
  </div>
</div>
