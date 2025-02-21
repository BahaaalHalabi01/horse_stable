import { invoke } from "@tauri-apps/api/core";
import { goto } from "$app/navigation";
import type { Horse, HorseCreate, Stable, StableCreate, User } from "$src/types";
import { Commands } from "$lib/ipc.defaults";
import { getUser } from "$src/routes/auth.svelte";

export const preventDefault = <T extends Event>(
  fn: (e: T) => void,
): ((e: T) => void) => {
  return (e: T) => {
    e.preventDefault();
    fn(e);
  };
};

type _State = {
  loading: boolean;
  horses: Horse[];
  stables: Stable[];
  result: any;
};

class GlobalState {

  private _state = $state<_State>({
    loading: false,
    horses: [],
    stables: [],
    result: null,
  });

  get stables() {
    return this._state.stables;
  }
  set stables(value: Stable[]) {
    this._state.stables = value;
  }

  get result() {
    return this._state.result;
  }
  set result(value: any) {
    this._state.result = value;
  }

  get horses() {
    return this._state.horses;
  }
  set horses(value: Horse[]) {
    this._state.horses = value;
  }

  get loading() {
    return this._state.loading;
  }

  set loading(value: boolean) {
    this._state.loading = value;
  }

  async get_horses() {
    this.loading = true;
    try {
      this.horses = await invoke<Horse[]>(Commands.list_all_horses)
    } catch (e) {
      alert(e)
    }
    this.loading = false;
  }

  async delete_horse(id: string) {
    try {
      this.horses = this.horses.filter((horse) => horse.id !== id.toString());
      // let res = await invoke<boolean>(Commands.remove_horse, { id })
      // if (!res) {
      //   alert("Failed to delete horse")
      //   return
      // }

      // this.result = res
    } catch (e) {
      alert(e);
    }
  }

  async edit_horse(horse: Horse) {
    try {
      await invoke<boolean>(Commands.edit_horse, { horse });
    } catch (e) {
      alert(e);
    }
  }
  async add_horse(stableId: number, horse: Partial<HorseCreate>) {
    try {
      let res = await invoke<Horse | undefined>(Commands.add_horse, { stableId, horse });
      if (res) {
        this.result = res
        this._state.horses.push(res)
      }
    } catch (e) {
      alert(e);
    }
  }
  async add_stable(stable: StableCreate) {
    this.loading = true;
    try {
      this._state.result = await invoke<boolean>(Commands.create_stable, { stable });
    } catch (e) {
      alert(e);
    }
    this.loading = false;
    goto("/");
  }

  async get_current_user(callback?: () => Promise<void>) {
    try {
      let { setCurrent } = getUser()
      let res = await invoke<User>(Commands.get_current_user)
      if (!res) {
        goto('/login')
      }

      await callback?.()
      //this is for testing and HMR 
      setCurrent(res)
    } catch (e) {
      alert(e)
    }
  }

  async register_user(user: User, callback?: () => Promise<void>) {
    try {
      let { setCurrent } = getUser()
      let res = await invoke<User>(Commands.register_user, { user })
      if (!res) {
        alert("Invalid credentials");
        return;
      }
      setCurrent(res);
      await callback?.()
    } catch (e) {
      alert(e);
    }
  }


  async get_stables() {
    this.loading = true;
    try {
      this._state.stables = await invoke<Stable[]>(Commands.list_stables)
    } catch (e) {
      alert(e)
    }
    this.loading = false;
  }
}

export const Ipc = new GlobalState();

console.log('Init Ipc State', Ipc)
