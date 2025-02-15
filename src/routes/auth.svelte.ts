import { writable } from "svelte/store";
import type { User } from "../types";

const currentStore = writable<User | undefined>(undefined);

export function getUser() {
  return {
    get user() {
      return currentStore;
    },
    setCurrent(value: User | undefined) {
      currentStore.set(value);
    },
  };
}
