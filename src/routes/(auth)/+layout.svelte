<script lang="ts">
    import { goto } from '$app/navigation'
    import { Ipc } from '$src/lib/ipc.svelte'
    import { getUser } from '../auth.svelte'

    let { children } = $props()

    let { user } = getUser()

    $effect(() => {
        void Ipc.get_current_user()
    })

    if (!$user) {
        goto('/login')
    }
</script>

{@render children()}
