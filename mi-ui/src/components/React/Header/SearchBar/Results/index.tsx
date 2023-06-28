import type { FC } from "react";
import type { UserBase } from "src/libs/types/user";
import BaseProfileCard from "@components/React/BaseProfileCard";

import styles from "./style.module.scss";

const Results: FC<{ results: UserBase[]; length?: number }> = ({
  results,
  length = 3,
}) => {
  return (
    <div className={styles.allResults}>
      {!!results.length && <h4>Matching users:</h4>}
      {!results.length && <h4>No users found.</h4>}
      {results.slice(0, length).map((user) => (
        <BaseProfileCard userData={user} key={user.id} />
      ))}
    </div>
  );
};
export default Results;
