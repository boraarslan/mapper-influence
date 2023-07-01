import React, { FC, useEffect, useState } from "react";
import { Beams } from "@components/SvgComponents";

import styles from "./style.module.scss";
import { useGlobalTheme } from "@states/theme";

type Props = {
  className?: string;
};

const DarkModeToggle: FC<Props> = ({ className }) => {
  const { setCss, theme: currentMode } = useThemeToggle();
  const { setTheme } = useGlobalTheme();

  const toggleMode = () => {
    const targetTheme = currentMode === "dark" ? "light" : "dark";
    setCss(targetTheme);
    setTheme(targetTheme);
  };

  return (
    <button
      aria-label="Toggle dark mode"
      className={`${styles.outerSlider} ${className}`}
      onClick={toggleMode}
    >
      <div className={`${styles.innerSlider} ${styles[currentMode]}`}>
        <div className={styles.colorFill} />
        <Beams className={styles.beam} color={"var(--buttonText)"} />
      </div>
    </button>
  );
};

export default DarkModeToggle;

const useThemeToggle = () => {
  const [theme, setTheme] = useState<"dark" | "light" | "none">("none");

  const setCss = (targetTheme: "dark" | "light" | "none") => {
    const root = window.document.documentElement;

    root.setAttribute("data-theme", targetTheme);
    localStorage.setItem("theme", targetTheme);

    if (typeof window !== "undefined") {
      localStorage.setItem("theme", targetTheme);
    }

    setTheme(targetTheme);
  };

  const getLocalColor = () => {
    if (window)
      switch (localStorage.getItem("theme")) {
        case "light":
          return "light";
        case "dark":
          return "dark";
        default:
          return window.matchMedia("(prefers-color-scheme: dark)").matches
            ? "dark"
            : "light";
      }
    return "none";
  };

  useEffect(() => {
    setCss(getLocalColor());
  }, []);

  return { theme, setCss };
};
