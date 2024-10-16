<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import type { User } from "$src/types/index";
  import { getUser } from "../auth.svelte";

  let { current, setCurrent } = getUser();
  let email = $state("");
  let password = $state("");

  async function login() {
    try {
      let res = await invoke<User>("login", { email, password });
      console.log(res);
      setCurrent(res);
      void goto("/");
    } catch (e) {
      alert(e);
    }
  }
</script>

<div class="container mx-auto py-16">
  <h1 class="text-3xl font-bold">Login</h1>
  <input
    type="text"
    bind:value={email}
    placeholder="Email"
    class="w-full p-2 border-2 border-gray-400 rounded-md"
  />
  <input
    type="password"
    bind:value={password}
    placeholder="Password"
    class="w-full p-2 border-2 border-gray-400 rounded-md"
  />
  <button class="w-full p-2 bg-blue-500 text-white rounded-md" onclick={login}
    >Login</button
  >
  <hr class="my-4" />
</div>
