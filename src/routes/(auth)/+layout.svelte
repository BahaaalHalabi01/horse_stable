<script lang="ts">
    import { goto } from '$app/navigation'
    import { Ipc } from '$src/lib/ipc.svelte'
    import { getUser } from '../auth.svelte'

    let { children } = $props()
    const { user } = getUser()

    $effect(() => {
        if (!$user) {
            goto('/login')
            return
        }

        Ipc.get_stables()
    })

    localStorage.setItem('user', JSON.stringify(user))

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
