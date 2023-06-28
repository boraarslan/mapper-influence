import { atom } from "nanostores";
import type { UserBase } from "src/libs/types/user";

export const userStore = atom<UserBase | null>(null);

export function setUser(newUser: UserBase | null) {
  userStore.set(newUser);
}

export function logout() {
  setUser(null);
}
