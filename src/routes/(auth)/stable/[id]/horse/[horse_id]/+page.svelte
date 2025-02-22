<script lang="ts">
    import { page } from '$app/state'
    import { Ipc } from '$lib/ipc.svelte'
    import { Calendar, Flag, Mars, Ruler, Venus, Weight } from 'lucide-svelte'

    const horse_id = page.params.horse_id

    let horse = $derived(Ipc.horses.find((h) => h.id === horse_id))

    // svelte-ignore state_referenced_locally
    if (horse === undefined) {
        throw new Error('Horse not found')
    }

    async function feedhorse() {
        await Ipc.feed_horse(horse_id)
    }
</script>

<div class="mx-auto container">
    <div class="flex">
        <div class="flex flex-col">
            <h1 class="text-4xl text-start pb-2 text-primary">{horse.name}</h1>
        </div>

        <figure class="size-52 ml-auto rounded">
            <img width="100%" src="/horse.jpg" alt="Stable" class="rounded" />
        </figure>
    </div>
    <div>
        <button class="btn btn-primary"> Clean the Horse </button>
        <button class="btn btn-primary" onclick={feedhorse}>
            Feed the Horse
        </button>
        <button class="btn btn-neutral"> Move to another Stable</button>
        <button class="btn btn-neutral"> Edit Info </button>
    </div>
    <div class="grid grid-cols-2 card-body">
        <span>{horse.breed}</span>
        <div
            class="badge !bg-transparent"
            style:color={horse.color}
            style:border-color={horse.color}
        >
            {horse.color}
        </div>
        <div class="flex items-center gap-2">
            <Flag class="w-5 h-5 " />
            <span>{horse.nationality}</span>
        </div>
        <div class="flex items-center gap-2">
            <Calendar class="w-5 h-5 " />
            <span>{horse.age} years</span>
        </div>
        <div class="flex items-center gap-2">
            {#if horse.gender === 'Male'}
                <Mars class="size-5 " />
            {:else}
                <Venus class="size-5" />
            {/if}
            <span>{horse.gender}</span>
        </div>
        <div class="flex items-center gap-2">
            <Weight class="w-5 h-5 " />
            <span>{horse.weight} kg</span>
        </div>
        <div class="flex items-center gap-2">
            <Ruler class="w-5 h-5 " />
            <span>{horse.height} cm (H)</span>
        </div>
        <div class="flex items-center gap-2">
            <Ruler class="w-5 h-5 " />
            <span>{horse.length} cm (L)</span>
        </div>
        <div class="flex items-center gap-2">
            <span>{horse.cleaness}</span>
            <span>%</span>
        </div>
        <div class="flex items-center gap-2">
            food
            <span>{horse.food}</span>
        </div>
        <div class="flex items-center gap-2">
            current_activity
            <span>{horse.current_activity}</span>
        </div>
        <div class="flex items-center gap-2">
            water
            <span>{horse.water}</span>
        </div>
    </div>
</div>
