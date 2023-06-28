import type { FC } from "react";
import type { UserBase } from "src/libs/types/user";
import Badge from "./Badge";
import Tooltip from "../Tooltip";

import styles from "./style.module.scss";

type Props = { userData: UserBase; className?: string };

const BaseProfileCard: FC<Props> = ({ userData, className = "" }) => {
  const Badges = userData.groups?.map((group) => (
    <Badge
      key={group.id}
      group={group}
    />
  ));

  return (
    <a
      href={`/profile/${userData.id}`}
      className={`${styles.cardWrapper} ${className}`}
    >
      <div className={styles.photoCell}>
        <img
          src={userData.avatarUrl}
          alt="Profile photo"
          className={styles.photo}
        />
        {Badges?.length && <div className={styles.badges}>{Badges}</div>}
      </div>
      <div className={styles.name}>{userData.username}</div>
      <div className={styles.influencedStat}>
        Influenced <span>1</span>
      </div>
      <div className={styles.rankedStat}>
        Ranked Maps <span>15</span>
      </div>
      <div className={styles.flag + " group"}>
        <img
          alt={userData.username + " is from " + userData.flag?.name}
          src={`https://flagcdn.com/${userData.flag?.code.toLowerCase()}.svg`}
        />
        
      </div>
      <Tooltip className="group-hover:opacity-100" >{userData.flag?.name}</Tooltip>
    </a>
  );
};

export default BaseProfileCard;
