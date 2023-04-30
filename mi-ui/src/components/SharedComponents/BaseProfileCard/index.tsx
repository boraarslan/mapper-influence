import Link from "next/link";
import { FC } from "react";
import { UserBase } from "@libs/types/user";
import Badge from "./Badge";

import styles from "./style.module.scss";

type Props = { userData: UserBase };

const BaseProfileCard: FC<Props> = ({ userData }) => {
  const Badges = userData.groups?.map((group) => (
    <Badge key={group.id} group={group} />
  ));

  return (
    <Link href={`/profile/${userData.id}`} passHref={true}>
      <a className={styles.cardWrapper}>
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
        <div className={styles.flag}>
          <img
            alt={userData.username + " is from " + userData.flag.name}
            src={`https://flagcdn.com/${userData.flag.code.toLowerCase()}.svg`}
          />
          <span className={styles.tooltip}>{userData.flag.name}</span>
        </div>
      </a>
    </Link>
  );
};

export default BaseProfileCard;
