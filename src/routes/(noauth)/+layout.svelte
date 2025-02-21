<script lang="ts">
    import { goto } from '$app/navigation'
    import { getUser } from '../auth.svelte'

    let { children } = $props()
    let { user } = getUser()  

    if ($user) {
        goto('/')
    }

</script>

{@render children()}
