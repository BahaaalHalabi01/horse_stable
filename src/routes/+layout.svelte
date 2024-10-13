<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { getUser } from "./auth.svelte";
    import "./app.css";

  let {children} = $props()

  let { current, setCurrent } = getUser();

  function handleLogout() {
    setCurrent(undefined);
    goto("/login");
  }

    $effect(() => {
      if (!current && $page.url.pathname !== "/login") {
        goto("/login");
      } 
    });
      

</script>

<main  class="grow">
{@render children()}
</main>
  <footer class="flex bg-primary w-full p-2 mt-auto">
  <a href="/contact-us" > Contact Us </a>
  <div class="flex gap-x-2 ml-auto">
  <button class="" onclick={() => window.location.reload()}>Refresh</button>
  {#if current?.id }
    <button class="" onclick={handleLogout}>Logout</button> 
  {/if}
  {#if current?.id}
    <div class="">
      <p>
        {current.email}
        {current.created_at}
        </p>
    </div>
  {/if}
</div>
</footer>
