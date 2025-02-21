<script lang="ts">
    import { Ipc } from '$src/lib/ipc.svelte'

    let { children } = $props()

    $effect.pre(() => {
          Ipc.get_stables()
    })

</script>

{@render children()}
