<script lang="ts">
  import type { FileEntry } from "@tauri-apps/plugin-fs";
  import { BaseDirectory, readDir } from "@tauri-apps/plugin-fs";
  import type { MouseEventHandler } from "svelte/elements";

  let entries: FileEntry[] = $state([]);
  let dir: string = $state("Rust/horse-stable/src-tauri");
  let loading: boolean = $state(false);

  $effect(() => {
    readDir(dir, {
      dir: BaseDirectory.Desktop,
      recursive: true,
    })
      .then((v) => {
        entries = v;
      })
      .catch((err) => {
        alert(err);
        goBack();
      })
      .finally(() => {
        loading = false;
      });
  });

  function goBack() {
    dir = dir.substring(0, dir.lastIndexOf("/"));
  }

  let handleDir: MouseEventHandler<HTMLButtonElement> = (event) => {
    dir += "/" + event.currentTarget.value;
  };
</script>

<div class="bg-white grid text-green-400 text-xl p-4 my-4">
  <div class="flex pb-4">
    {#if dir !== "Rust/horse-stable"}
      <button
        class="bg-green-400 text-white rounded-md w-fit px-2 mr-2"
        onclick={() => {
          loading = true;
          goBack();
        }}>{"<"}</button
      >
    {/if}
    {dir}
  </div>
  {#if loading}
    <div class="flex justify-center">
      <div class="spinner-border text-green-400 w-10 h-10" role="status">
        <span class="visually-hidden">Loading...</span>
      </div>
    </div>
  {/if}
  {#if !loading}
    <ul class="space-y-2 list-none">
      {#each entries as entry}
        <li class="border p-2 text-black">
          {entry.path}
          {#if Number(entry.children?.length) > 0}
            <button onclick={handleDir} value={entry.name}> more </button>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</div>
