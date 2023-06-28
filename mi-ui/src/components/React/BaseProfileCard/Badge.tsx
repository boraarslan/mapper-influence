import type { FC } from "react";
import type { Group } from "src/libs/types/IOsuApi";

import styles from "./style.module.scss";

type Props = { group: Group };
const Badge: FC<Props> = ({ group }) => {
  return (
    <span
      className={styles.badge}
      style={{ color: group.colour, borderColor: group.colour }}
    >
      {group.short_name}
      <span className={styles.tooltip}>{group.name}</span>
    </span>
  );
};
export default Badge;
