<script lang="ts">
    import { Ipc, preventDefault } from '$lib/ipc.svelte'

    let form: HTMLFormElement | null = $state(null)

    const onsubmit = preventDefault(async () => {
        if (!form) return
        const data = new FormData(form)
        await Ipc.add_stable({
            name: data.get('name') as string,
            address: data.get('address') as string,
            monthly_fee: Number(data.get('monthly_fee') as string),
        })
    })
</script>

<div class="mx-auto container">
    <h1 class="text-4xl text-center pb-2 text-primary">
        Creating a new Stable
    </h1>
    <h2 class="text-3xl text-center pb-10 text-base-content italic">
        Pass all the required information and we will create a new stable for
        you.
    </h2>

    <form class=" bg-base-300 card" {onsubmit} bind:this={form}>
        <div class="card-body">
            <input
                type="text"
                name="name"
                required
                minlength="3"
                placeholder="Name"
                class="input input-bordered w-full"
            />
            <input
                type="text"
                required
                autocomplete="address-line1"
                minlength="5"
                name="address"
                placeholder="Address"
                class="input input-bordered w-full"
            />
            <input
                required
                type="number"
                name="monthly_fee"
                placeholder="Monthly Fee"
                class="input input-bordered w-full"
            />

            <div class="ml-auto gap-4 flex">
                <button class="btn w-fit btn-error" type="reset">Reset</button>
                <button class="btn btn-primary w-fit ml-auto" type="submit"
                    >Submit</button
                >
            </div>
        </div>
    </form>
</div>
