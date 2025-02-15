<script lang="ts">
    import type { Horse } from '../types'
    import HorseCard from './Horse-Card.svelte'
    import { Ipc } from './ipc.svelte'

    let name = $state('')
    let default_horse: Horse = {
        id: '',
        name: 'Jimmy',
        breed: 'English',
        color: 'Brown',
        nationality: 'English',
        age: 10,
        gender: 'Male',
        weight: 200,
        height: 140,
        length: 250,
    }
    let horses = $derived(Ipc.horses)

    $inspect(horses)

    async function add_horse() {
        void Ipc.add_horse(default_horse)
    }

    async function get_horses() {
        void Ipc.get_horses()
    }

    $effect(() => {
    void get_horses()
    })
</script>

<div class="py-2 w-full max-w-lg mx-auto my-4 grid rounded gap-4 px-4">
    <input
        id="greet-input"
        placeholder="Enter a name for horse..."
        bind:value={name}
        class="p-2 placeholder:text-primary placeholder:italic rounded"
    />
    <button
        onclick={add_horse}
        class="bg-primary p-2 rounded font-bold disabled:bg-primary/70 disabled:cursor-not-allowed"
        disabled={false}>Add</button
    >
</div>
<div
    class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
>
    {#each horses as _, i}
        <HorseCard bind:horse={horses[i]} />
    {/each}
</div>
