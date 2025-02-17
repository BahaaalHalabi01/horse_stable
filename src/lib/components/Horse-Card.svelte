<script lang="ts">
    import type { Horse } from '$src/types'
    import type { EventHandler } from 'svelte/elements'
    import { Ipc } from '$lib/ipc.svelte'
    import { confirm } from '@tauri-apps/plugin-dialog'
    import {
        Calendar,
        Flag,
        Ruler,
        Weight,
        Mars,
        Venus,
        EllipsisVertical,
    } from 'lucide-svelte'
    import { page } from '$app/state'

    let stable_id = page.params.id

    let {
        horse = $bindable(),
    }: {
        horse: Horse
    } = $props()

    let delete_horse = async function () {
        if (await confirm('Are you sure you want to delete this horse?')) {
            void Ipc.delete_horse(horse.id)
        }
    }

    let edit_horse: EventHandler<MouseEvent, HTMLButtonElement> =
        async function () {
            void Ipc.edit_horse(horse)
        }
</script>

<div class="card bg-base-300 shadow-sm relative">
    <div class="dropdown z-10 absolute right-4 top-4">
        <div
            tabindex="0"
            role="button"
            class="btn text-primary btn-ghost w-fit p-2"
        >
            <EllipsisVertical class="size-5" />
        </div>
        <ul
            role="menu"
            tabindex="0"
            class="dropdown-content menu bg-base-200 rounded-box z-1 w-32 p-2 shadow-sm text-sm space-y-2"
        >
            <li>
                <button class="btn btn-primary" onclick={edit_horse}
                    >Edit</button
                >
            </li>
            <li>
                <button class="btn btn-error" onclick={delete_horse}
                    >Delete</button
                >
            </li>
        </ul>
    </div>
    <a
        class="contents"
        href={`/stable/${stable_id}/horse/${horse.id}`}
        data-sveltekit-preload-code="hover"
    >
        <figure class="h-64 relative">
            <figcaption
                class="text-2xl font-bold text-primary p-4 absolute left-0 top-0"
            >
                {horse.name}
            </figcaption>
            <img width="100%" src="/horse.jpg" alt="Stable" />
        </figure>
    </a>
    <div class="grid grid-cols-2 card-body">
        <div class="flex items-center gap-2">
            <span>{horse.breed}</span>
        </div>
        <div class="flex items-center gap-2">
            <div
                class="badge !bg-transparent"
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
