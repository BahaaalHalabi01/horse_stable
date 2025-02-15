import { invoke } from "@tauri-apps/api/core";
import { goto } from "$app/navigation";
import type { Horse } from "$src/types";
import { Commands } from "$lib/ipc.defaults";

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
};
class GlobalState {
  private _state = $state<_State>({
    loading: false,
    horses: [],
  });

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
      let res = await invoke<boolean>(Commands.remove_horse, { id });
      if (res) {
        this.horses = this.horses.filter((horse) => horse.id !== id.toString());
      }
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
  async add_horse(horse: Horse) {
    try {
      await invoke<boolean>(Commands.add_horse, { horse });
    } catch (e) {
      alert(e);
    }
  }
}

export const Ipc = new GlobalState();
