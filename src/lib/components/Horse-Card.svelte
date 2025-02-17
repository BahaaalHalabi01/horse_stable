<script lang="ts">
    import type { Horse } from '$src/types'
    import type { EventHandler } from 'svelte/elements'
    import { Ipc } from '$lib/ipc.svelte'
    import {
        Calendar,
        Flag,
        Ruler,
        UserIcon,
        Weight,
        Mars,
        Venus,
    } from 'lucide-svelte'

    let {
        horse = $bindable(),
    }: {
        horse: Horse
    } = $props()

    let editMode = $state(false)

    let delete_horse: EventHandler<MouseEvent, HTMLButtonElement> = function (
        e
    ) {
        void Ipc.delete_horse(e.currentTarget.value)
    }
    let handle_edit: EventHandler<MouseEvent, HTMLButtonElement> =
        async function () {
            void Ipc.edit_horse(horse)
        }
</script>

<div class="card bg-base-300 shadow-sm">
    <figure class="h-64 relative">
        <figcaption
            class="text-2xl font-bold text-primary p-4 absolute left-0 top-0"
        >
            {horse.name}
        </figcaption>
        <img width="100%" src="/horse.jpg" alt="Stable" />
    </figure>
    <div class="grid grid-cols-2 card-body">
        <div class="flex items-center gap-2">
            <span>{horse.breed}</span>
        </div>
        <div class="flex items-center gap-2">
            <div
                class="badge !bg-transparent "
                style:color={horse.color}
                style:border-color={horse.color}
            >
                {horse.color}
            </div>
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
    </div>
</div>

