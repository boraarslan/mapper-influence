import { FC } from "react";
import BaseProfileCard from "@components/SharedComponents/BaseProfileCard";
import { UserBaseResponse } from "@services/user";

import styles from "./style.module.scss";

type Props = { mentions: UserBaseResponse[]; open?: boolean };
const MentionList: FC<Props> = ({ mentions, open }) => {
  return (
    <div
      className={styles.mentionList}
      style={!open ? { display: "none" } : {}}
    >
      <div className={styles.mentionGrid}>
        {mentions.map((user) => (
          <BaseProfileCard key={user.id} userId={user.id} />
        ))}
      </div>
      {mentions.length === 0 && <span>{"No mentions :("}</span>}
    </div>
  );
};
export default MentionList;
