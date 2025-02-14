import type { User } from "../types";

let current = $state<User | undefined>(undefined);

export function getUser() {
  return {
    get current() {
      return current;
    },
    setCurrent(value: typeof current) {
      current = value;
    },
  };
}
