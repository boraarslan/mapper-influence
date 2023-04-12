import BaseProfileCard from "@components/SharedComponents/BaseProfileCard";
import React, { FC } from "react";
import { Influence } from "@libs/types/influence";
import EditableDescription from "../EditableDescription";
import InfluenceType from "./InfluenceType";
import MapCarousel from "@components/SharedComponents/MapCarousel/SingleItem";

import styles from "./style.module.scss";

const InfluenceElement: FC<{
  influenceData: Influence;
  editable?: boolean;
}> = ({ influenceData, editable }) => {
  return (
    <>
      <div className={styles.influenceRow}>
        <div className={styles.cardSide}>
          <BaseProfileCard userData={influenceData.profileData} />
          <InfluenceType editable influenceType={influenceData.type} />
        </div>
        <EditableDescription
          className={styles.description}
          label={`Description textarea for ${influenceData.profileData.username}`}
          description={influenceData.description}
          editable={editable}
          placeholder={"Describe your influence here."}
        />
        <div className={styles.maps}>
          <h4>Featured Maps</h4>
          <MapCarousel mapList={influenceData.maps || []} />
        </div>
      </div>
    </>
  );
};

export default InfluenceElement;
