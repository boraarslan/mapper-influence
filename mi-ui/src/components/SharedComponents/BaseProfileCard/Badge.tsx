import type { FC } from "react";
import { Group } from "@libs/types/IOsuApi";
import Tooltip from "../Tooltip";

import styles from "./style.module.scss";
import { useGlobalTooltip } from "src/states/globalTooltip";

type Props = { group: Group };
const Badge: FC<Props> = ({ group }) => {
  const { activateTooltip } = useGlobalTooltip();

  return (
    <span
      className={styles.badge}
      style={{ color: group.colour, borderColor: group.colour }}
      onMouseEnter={(e) => activateTooltip(group.name, e.currentTarget)}
    >
      {group.short_name}
    </span>
  );
};
export default Badge;
