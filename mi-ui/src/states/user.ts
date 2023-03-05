import { create } from "zustand";
import { devtools, persist } from "zustand/middleware";
import { User, UserBase } from "@libs/types/user";

type SessionStore = {
  authKey?: string;
  user?: UserBase;
  login: (user: UserBase, key: string) => void;
  logout: () => void;
};

export const useSessionStore = create<SessionStore>()(
  devtools(
    persist(
      (set) => ({
        login: (user, key) => set({ user: user, authKey: key }),
        logout: () => set({ user: undefined, authKey: undefined }),
      }),
      { name: "userStore" }
    )
  )
);
