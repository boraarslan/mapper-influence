import { useEffect, type FC, type ReactNode } from "react";

import styles from "./style.module.scss";

type Props = {
  className?: string;
  children?: ReactNode;
};
const Tooltip: FC<Props> = ({ children, className }) => {
  useEffect(() => {
    var element = document.getElementById("tooltip");

    function handleMouse(e: MouseEvent) {
      if (!element) return;
      const padding = 16;
      const deadzone = 16;

      // X axis
      if (element.clientWidth / 2 + e.pageX + deadzone > window.innerWidth)
        element.style.left = e.pageX - element.clientWidth - padding + "px";
      else element.style.left = e.pageX - element.clientWidth / 2 + "px";

      // Y axis
      if (e.clientY - element.clientHeight - deadzone <= 0)
        element.style.top = e.pageY + padding + "px";
      else element.style.top = e.pageY - element.clientHeight - padding + "px";
    }

    document.addEventListener("mousemove", handleMouse);
    return () => {
      document.removeEventListener("mousemove", handleMouse);
    };
  }, []);

  return (
    <span
      className={`${styles.tooltip} ${className}`}
      id="tooltip"
    >
      {children}
    </span>
  );
};

export default Tooltip;
