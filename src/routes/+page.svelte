<script lang="ts">
    import { Ipc } from '$lib/ipc.svelte'

    void Ipc.get_horses()
</script>

<div class="max-w-lg mx-auto container">
    <h1 class="text-4xl text-center pb-2 text-primary">Welcome</h1>
    <h2 class="text-3xl text-center pb-10 text-base-content italic">
        Built by Tauri!(Rust BTW)
    </h2>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 place-items-center">
        <div class="card bg-base-300 w-96 shadow-sm">
            <figure>
                <img
                    src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
                    alt="Shoes"
                />
            </figure>
            <div class="card-body">
                <h2 class="card-title">
                    Card Title
                    <div class="badge badge-secondary">NEW</div>
                </h2>
                <p>
                    A card component has a figure, a body part, and inside body
                    there are title and actions parts
                </p>
                <div class="card-actions justify-end">
                    <div class="badge badge-outline">Fashion</div>
                    <div class="badge badge-outline">Products</div>
                </div>
            </div>
        </div>
    </div>
</div>
