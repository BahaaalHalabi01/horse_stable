<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import type { Horse } from "../types";

  let name = $state("");
  let horses: Horse[] = $state([]);
  let progress = $state(0);
  let progress_files = $state(0);
  let files_size = $state(0);
  let files_count = $state(0);

  async function greet() {
    await invoke<Horse>("load");
    // let v = await invoke<Horse>("get_horse", { name });

    // let temp = [...horses];
    // temp.push(v);
    // horses = temp;
  }

  $effect(() => {
    async function events() {
      return await listen("download-progress", (e: any) => {

        const { filesWritten, bytesWritten } = e.payload;
        progress = bytesWritten;
        progress_files =filesWritten;
      });
    }

    async function start_download() {
      progress = 0;
      return await listen("download-started", (e: any) => {
        console.log(e);
        const { filesSize:fsize, filesCount: fcount } = e.payload;
        files_size = fsize;
        files_count = fcount
      });
    }

    async function end_download() {
      return await listen("download-finished", (e: any) => {

        console.log(e);
        progress = files_size;
        progress_files = files_count;
      });
    }

    let x = start_download();
    let z = events();
    let y = end_download();
    return () => {
      x.then((x) => x());
      z.then((x) => x());
      y.then((x) => x());
    };
  });
</script>

<div class="py-2 w-fit mx-auto my-4 grid rounded gap-4 px-4">
  <!-- <input -->
  <!--   id="greet-input" -->
  <!--   placeholder="Enter a name for horse..." -->
  <!--   bind:value={name} -->
  <!--   class="p-2 placeholder:text-primary placeholder:italic rounded" -->
  <!-- /> -->
  <button
    onclick={greet}
    class="bg-primary p-2 rounded font-bold disabled:bg-primary/70 disabled:cursor-not-allowed"
    disabled={false}>Zip test</button
  >
  <div class="flex gap-2 items-center bg-white rounded p-2 relative">
    <span class="z-10">
      {(progress / 1024 / 1024).toFixed(2)} / {(files_size / 1024 / 1024).toFixed(2)} MB
    </span>
    <span class="z-10">
      {progress_files} / {files_count} files
    </span>
    <div
      class="absolute top-0 left-0 h-full bg-primary max-w-full"
      style={`width: ${Math.min((progress / files_size) * 100, 100)}%`}
    ></div>
  </div>
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
