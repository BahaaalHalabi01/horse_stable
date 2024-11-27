<script lang="ts">
    import { goto } from "$app/navigation";
    import { getUser } from "./auth.svelte";
    import "./app.css";

  let {children} = $props()

  let user = getUser();

  function handleLogout() {
    user.setCurrent(undefined);
    goto("/login");
  }

    console.log(user.current);

</script>

<main  class="grow">
{@render children()}
</main>
  <footer class="flex bg-primary w-full p-2 mt-auto">
  <a href="/contact-us" > Contact Us </a>
  <a href="/register" > Register </a>
  <a href="/" > Home </a>
  <div class="flex gap-x-2 ml-auto">

  <button class="" onclick={() => window.location.reload()}>Refresh</button>
  {#if user.current?.id }
    <button class="" onclick={handleLogout}>Logout</button> 
    <a href="/horses" > Horses </a>
    <div class="">
      <p>
        {user.current.email}
        {user.current.created_at}
        </p>
    </div>
  {/if}
</div>
</footer>
