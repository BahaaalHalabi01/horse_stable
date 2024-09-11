<script lang="ts">
    import Files from "$lib/Files.svelte";
import Greet from "../lib/Greet.svelte";
</script>

<div class="container mx-auto py-16">
  <h1 class="text-4xl text-center pb-2 text-primary">Welcome</h1>
  <h2 class="text-3xl text-center pb-10 text-secondary italic">Built by Tauri!(Rust BTW)</h2>

  <Greet />
  <Files />

</div>

<style>

</style>
