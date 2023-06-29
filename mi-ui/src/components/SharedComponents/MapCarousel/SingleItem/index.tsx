import { FC, useCallback, useEffect, useState } from "react";
import { Carousel } from "react-responsive-carousel";

import { MapInfo } from "@libs/types/user";
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
      showThumbs={false}
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
