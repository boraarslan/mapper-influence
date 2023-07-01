import { create } from "zustand";

type DarkModeState = {
  theme: "dark" | "light";
  setTheme: (theme: "dark" | "light") => void;
};

export const useGlobalTheme = create<DarkModeState>((set) => ({
  theme: "light",
  setTheme: (theme) => set({ theme }),
}));
