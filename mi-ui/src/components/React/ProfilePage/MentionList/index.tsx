import type { FC } from "react";
import type { UserBase } from "src/libs/types/user";
import BaseProfileCard from "@components/React/BaseProfileCard";

import styles from "./style.module.scss";

type Props = { mentions: UserBase[] };
const MentionList: FC<Props> = ({ mentions }) => {
  return (
    <div className={styles.mentionList}>
      <div className={styles.mentionGrid}>
        {mentions.map((user) => (
          <BaseProfileCard key={user.id} userData={user} />
        ))}
      </div>
      {mentions.length === 0 && <span>{"No mentions :("}</span>}
    </div>
  );
};
export default MentionList;
