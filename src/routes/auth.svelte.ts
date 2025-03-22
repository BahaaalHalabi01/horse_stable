import { writable } from "svelte/store";
import type { User } from "../types";

const currentStore = writable<User | undefined>(undefined);
let _user: User | undefined = undefined
currentStore.subscribe((v) => _user = v)

export function getUser() {
  return {
    get _user() {
      return _user;
    },
    get user() {
      return currentStore;
    },
    setCurrent(value: User | undefined) {
      currentStore.set(value);
    },
  };
}
