<script lang="ts">
    import HorseCard from './Horse-Card.svelte'
    import { Ipc } from './ipc.svelte'


</script>

<div
    class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
>
    {#each Ipc.horses as _, i}
        <HorseCard bind:horse={Ipc.horses[i]} />
    {/each}
</div>
