<script lang="ts">
    import { page } from '$app/state'
    import HorseCard from '$lib/components/Horse-Card.svelte'
    import { Ipc } from '$lib/ipc.svelte'

    const stable_id = page.params.id
    let stable = $derived(
        Ipc.stables.find((stable) => String(stable.id) === stable_id)
    )

    void Ipc.get_horses()

</script>

<div class="flex">
    <div class="flex flex-col">
        <h1 class="text-4xl text-start pb-2 text-primary">{stable?.name}</h1>
        <h2 class="text-3xl pb-10 text-base-content italic">
            {stable?.address}
        </h2>
        <div>
            <a class="btn btn-primary" href={`/stable/${stable_id}/horse-add`}> Add a new Horse </a>
        </div>
    </div>

    <figure class="size-52 ml-auto">
        <img width="100%" src="/stable-card.jpg" alt="Stable" class="rounded" />
    </figure>
</div>

<div
    class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
>
    {#each Ipc.horses as _, i}
        <HorseCard bind:horse={Ipc.horses[i]} />
    {/each}
</div>
