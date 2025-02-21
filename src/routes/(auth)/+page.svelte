<script lang="ts">
    import { Ipc } from '$lib/ipc.svelte'
    import { formatter } from '$src/lib/ipc.defaults'

    let stables = $derived(Ipc.stables)
    const currency = formatter.currency

</script>

<div class="mx-auto container">
    <h1 class="text-4xl text-center pb-2 text-primary">Welcome</h1>
    <h2 class="text-3xl text-center pb-10 text-base-content italic">
        Built by Tauri!(Rust BTW)
    </h2>
    <div class="flex w-full pb-4">
        <a class="btn btn-primary" href="/stable-create">
            Create a new Stable
        </a>
    </div>
    <section class="">
        <div
            class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 place-items-center md:place-items-start"
        >
            {#each stables as stable}
                <div class="card bg-base-300 shadow-sm">
                    <a href={`/stable/${stable.id}`} class="contents">
                        <figure>
                            <img
                                width="100%"
                                src="/stable-card.jpg"
                                alt="Stable"
                            />
                        </figure>
                    </a>
                    <div class="card-body">
                        <h2 class="card-title">
                            {stable.name}
                        </h2>
                        <p>
                            {stable.address}
                        </p>
                        <div class="card-actions justify-end">
                            <div class="badge badge-outline">
                                {currency.format(stable.monthly_fee)}
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </section>
</div>
