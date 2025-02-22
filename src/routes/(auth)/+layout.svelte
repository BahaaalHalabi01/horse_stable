<script lang="ts">
    import { Ipc } from '$src/lib/ipc.svelte'

    let { children } = $props()

    $effect.pre(() => {
             Ipc.get_stables()
    })

    let loading = $derived(Ipc.loading)
</script>

{#if loading}
    <div
        class="fixed top-1/2 left-1/2 flex items-center justify-center text-3xl -translate-x-1/2 -translate-y-1/2 italic"
    >
        <span class="loading loading-ring w-28 text-primary"></span>
    </div>
{/if}
<div class:content={!loading} class:opacity-20={loading}>
    {@render children()}
</div>
