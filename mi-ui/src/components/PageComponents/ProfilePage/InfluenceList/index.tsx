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

  const firstInfRef = useRef<HTMLDivElement>(null);
  const firstIntersection = useIntersectionObserver(firstInfRef, {
    threshold: 1,
  });
  const lastInfRef = useRef<HTMLDivElement>(null);
  const lastIntersection = useIntersectionObserver(lastInfRef, {
    threshold: 1,
  });
  const firstGradient = !firstIntersection?.isIntersecting;
  const lastGradient = !lastIntersection?.isIntersecting;

  const InfluenceCards = influences?.map((influence, i) => {
    let ref;
    if (i === 0) ref = firstInfRef;
    if (i === influences.length - 1) ref = lastInfRef;
    return (
      <InfluenceElement
        ref={ref}
        key={influence.from_id}
        influenceData={influence}
        editable={editable}
      />
    );
  });

  const rootClass = `${styles.mapperInfluences} ${
    firstGradient ? styles.firstGradient : ""
  } ${lastGradient ? styles.lastGradient : ""}`;

  return (
    <div className={rootClass} style={!open ? { display: "none" } : {}}>
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
