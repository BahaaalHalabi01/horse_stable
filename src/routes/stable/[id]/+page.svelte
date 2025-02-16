<script lang="ts">
    import { page } from '$app/state'
    import Horses from '$src/lib/Horses.svelte'
    import { Ipc, preventDefault } from '$src/lib/ipc.svelte'
    import type { HorseCreate } from '$src/types'

    let form: HTMLFormElement | null = $state(null)
    let stable_id = page.params.id

    async function get_horses() {
        void Ipc.get_horses()
    }

    void get_horses()

    const onsubmit = preventDefault(async () => {
        if (!form) return
        const data = new FormData(form)
        //@todo this has to just take a form and the stuff do inside the function
        await Ipc.add_horse(Number(stable_id), {
            name: data.get('name')?.toString(),
            breed: data.get('breed')?.toString(),
            color: data.get('color')?.toString(),
            nationality: data.get('nationality')?.toString(),
            age: Number(data.get('age') as string),
            gender: data.get('gender') as HorseCreate['gender'],
            weight: Number(data.get('weight') as string),
            height: Number(data.get('height') as string),
            length: Number(data.get('length') as string),
        })
    })
</script>

<form
    class="py-2 w-full mx-auto my-4 grid rounded gap-4 px-4"
    bind:this={form}
    {onsubmit}
>
    <div class="grid md:grid-cols-2 gap-4 grid-cols-1 place-items-stretch">
        <input
            name="name"
            placeholder="Enter a name for horse..."
            class="input"
            required
        />
        <input name="breed" placeholder="Breed" class="input" required />
        <input name="color" placeholder="Color" class="input" required />
        <input
            name="nationality"
            placeholder="Nationality"
            class="input"
            required
        />
        <input name="age" placeholder="Age" class="input" required />

        <select class="select" name="gender" required>
            <option disabled selected>Pick a gender</option>
            <option>Male</option>
            <option>Female</option>
        </select>
        <input
            name="weight"
            placeholder="Weight"
            class="input"
            type="number"
            required
        />
        <input
            name="height"
            placeholder="Height"
            class="input"
            type="number"
            required
        />
        <input
            name="length"
            placeholder="Length"
            class="input"
            type="number"
            required
        />
    </div>

    <div class="flex grow gap-4 w-1/2 ml-auto">
        <button
            type="reset"
            class="btn btn-neutral grow"
            disabled={false}>Reset</button
        >

        <button
            type="submit"
            class="btn btn-primary grow"
            disabled={false}>Add</button
        >
    </div>
</form>
<Horses />
