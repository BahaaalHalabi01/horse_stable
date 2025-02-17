import type { PageLoad } from "./$types"
import { Ipc } from '$lib/ipc.svelte'

export const load: PageLoad = async ({ params }) => {


  const horse_id = params.horse_id
  await Ipc.get_horses()

  let horse = Ipc.horses.find((h) => h.id === horse_id)
  if (horse) {
    return { horse }
  }

  throw new Error('Horse not found')


}
