import { FC } from "react";
import { useGetInfluences } from "@services/influence";
import InfluenceElement from "./InfluenceElement";

import styles from "./style.module.scss";

const InfluenceList: FC<{
  userId?: string | number;
  open?: boolean;
}> = ({ userId, open }) => {
  const editable = !userId;

  const { data: influences } = useGetInfluences(userId);

  const InfluenceCards = influences?.map((influence) => (
    <InfluenceElement
      key={influence.from_id}
      influenceData={influence}
      editable={editable}
    />
  ));

  return (
    <div
      className={styles.mapperInfluences}
      style={!open ? { display: "none" } : {}}
    >
      <div className={styles.scrollWrapper}>
        {InfluenceCards}
        {!!influences?.length && (
          <span>
            {`This person is unique!`}
            <br />
            {`...Or they haven't added anyone yet.`}
          </span>
        )}
      </div>
    </div>
  );
};

export default InfluenceList;
