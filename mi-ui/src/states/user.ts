import { create } from "zustand";
import { devtools, persist } from "zustand/middleware";
import { UserBase } from "@libs/types/user";

type SessionStore = {
  user?: UserBase;
  login: (user: UserBase) => void;
  logout: () => void;
};

export const useSessionStore = create<SessionStore>()(
  devtools(
    persist(
      (set) => ({
        login: (user) => set({ user: user }),
        logout: () => set({ user: undefined }),
      }),
      { name: "userStore" }
    )
  )
);
