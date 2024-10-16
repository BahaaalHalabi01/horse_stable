<script lang="ts">
    import { goto } from "$app/navigation";
  import type { User } from "$src/types/index";
  import { getUser } from "../auth.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let email = $state("");
  let password = $state("");
  let { current, setCurrent } = getUser();
  async function register() {
    let user: User = {
      id: "",
      email,
      password,
      username: email,
      created_at: 0,
      updated_at: 0,
    };

    try {
      let res = await invoke<User>("register_user", { user });
      console.log(res);
      setCurrent(res);
      void goto("/");
    } catch (e) {
      alert(e);
    }
  }
</script>

<h1 class="text-3xl font-bold">Register</h1>
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
<button class="w-full p-2 bg-blue-500 text-white rounded-md" onclick={register}
  >Register</button
>
