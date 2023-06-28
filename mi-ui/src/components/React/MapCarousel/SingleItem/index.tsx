import type { FC } from "react";
import type { MapInfo } from "src/libs/types/user";
import { Carousel } from "react-responsive-carousel";
import MapCard from "../../MapCard";

import styles from "./style.module.scss";
import "react-responsive-carousel/lib/styles/carousel.min.css";

const SingleItemCarousel: FC<{
  mapList: MapInfo[];
  className?: string;
  editable?: boolean;
}> = ({ mapList, editable, className = "" }) => {
  return (
    <Carousel
      className={`${styles.carousel} ${className}`}
      showStatus={false}
      showArrows={false}
      transitionTime={0}
    >
      {mapList.map((item) => (
        <div key={item.mapUrl} className={styles.slide}>
          <MapCard {...item} />
        </div>
      ))}
    </Carousel>
  );
};

export default SingleItemCarousel;
