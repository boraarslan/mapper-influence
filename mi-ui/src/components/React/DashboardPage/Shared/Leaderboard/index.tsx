import type { FC } from "react";
import type { LeaderboardType } from "src/libs/types/influence";
import BaseProfileCard from "@components/React/BaseProfileCard";

import styles from "./style.module.scss";

const Leaderboard: FC<{ topList: LeaderboardType[]; className?: string }> = ({
  topList,
  className,
}) => {
  return (
    <div className={`${styles.wrapper} ${className}`}>
      <h2>Top Influencers</h2>
      <div className={styles.list}>
        {topList.map((rowData) => (
          <div
            key={rowData.user.id}
            className={styles.row}
          >
            <BaseProfileCard userData={rowData.user} />
            <div className={styles.number}>
              <span>{rowData.number}</span>
              <span>{`Mention${rowData.number !== 1 ? "s" : ""}`}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Leaderboard;
