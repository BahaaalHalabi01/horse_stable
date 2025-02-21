<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import type { User } from "$src/types/index";
  import { getUser } from "$src/routes/auth.svelte";

  let { setCurrent } = getUser();
  let email = $state("");
  let password = $state("");

  async function login() {
    try {
      let res = await invoke<User>("login", { email, password });
      if (!res) {
        alert("Invalid credentials");
        return;
      }
      setCurrent(res);
      void goto("/");
    } catch (e) {
      alert(e);
    }
  }
</script>

<div class=" mx-auto py-16 space-y-4 max-w-md">
  <h1 class="text-3xl font-bold">Login</h1>
  <input
    type="text"
    bind:value={email}
    placeholder="Email"
    class="input w-full"
  />
  <input
    type="password"
    bind:value={password}
    placeholder="Password"
    class="input w-full"
  />
  <button class="btn w-full btn-primary" onclick={login}
    >Login</button
  >
  <hr />
  <p class="text-center text-sm">
    Don't have an account? <a href="/register">Register</a>
  </p>
</div>
