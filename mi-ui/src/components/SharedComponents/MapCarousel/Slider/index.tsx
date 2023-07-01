import { FC, useCallback, useEffect, useState } from "react";
import useEmblaCarousel from "embla-carousel-react";
import MapCard from "@components/SharedComponents/MapCard";
import { FeaturedMapsResponse } from "@services/user";

import styles from "./style.module.scss";

const SliderCarousel: FC<{ mapList: FeaturedMapsResponse[] }> = ({
  mapList,
}) => {
  const [emblaRef, embla] = useEmblaCarousel({
    skipSnaps: true,
    inViewThreshold: 1,
    align: "start",
  });

  // States for showing scrollable gradient
  const [hasPrev, setHasPrev] = useState(false);
  const [hasNext, setHasNext] = useState(false);

  const onScroll = useCallback(() => {
    if (!embla) return;
    setHasPrev(embla.scrollProgress() > 0);
    setHasNext(embla.scrollProgress() < 100);
  }, [embla]);

  useEffect(() => {
    if (!embla) return;
    embla.on("scroll", onScroll);
    onScroll();
  }, [embla, onScroll]);

  return (
    <div ref={emblaRef} className={styles.viewport}>
      <div>
        {mapList.map((item) => (
          <div key={item.beatmapset.id} className={styles.slide}>
            <MapCard map={item} />
          </div>
        ))}
      </div>
      <div
        className={`${styles.prevGradient} ${hasPrev ? styles.visible : ""}`}
      />
      <div
        className={`${styles.nextGradient} ${hasNext ? styles.visible : ""}`}
      />
    </div>
  );
};

export default SliderCarousel;
