import type { FC } from "react";
import type { UserBase } from "@libs/types/user";
import Link from "next/link";
import Badge from "./Badge";
import { useGlobalTooltip } from "src/states/globalTooltip";

import styles from "./style.module.scss";

type Props = { userData: UserBase; className?: string };

const BaseProfileCard: FC<Props> = ({ userData, className = "" }) => {
  const { activateTooltip } = useGlobalTooltip();

  const Badges = userData.groups?.map((group) => (
    <Badge key={group.id} group={group} />
  ));

  return (
    <Link
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
      {userData.flag && (
        <div
          className={styles.flag}
          onMouseEnter={(e) =>
            userData.flag &&
            activateTooltip(userData.flag.name, e.currentTarget)
          }
        >
          <img
            alt={userData.username + " is from " + userData.flag.name}
            src={`https://flagcdn.com/${userData.flag.code.toLowerCase()}.svg`}
          />
        </div>
      )}
    </Link>
  );
};

export default BaseProfileCard;
