import { FC, useRef } from "react";
import { useAutoAnimate } from "@formkit/auto-animate/react";
import { useGetInfluences } from "@services/influence";
import InfluenceElement from "./InfluenceElement";

import styles from "./style.module.scss";
import { useIntersectionObserver } from "usehooks-ts";

const InfluenceList: FC<{
  userId?: string | number;
  open?: boolean;
}> = ({ userId, open }) => {
  const editable = !userId;

  const { data: influences } = useGetInfluences(userId);
  const [animateRef] = useAutoAnimate({ easing: "ease-out", duration: 200 });

  const InfluenceCards = influences?.map((influence, i) => (
    <InfluenceElement
      key={influence.from_id}
      influenceData={influence}
      editable={editable}
    />
  ));

  return (
    <div
      className={styles.mapperInfluences}
      style={!open ? { display: "none" } : {}}>
      <div className={styles.scrollWrapper} ref={animateRef}>
        {InfluenceCards}
        {!influences?.length && (
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
