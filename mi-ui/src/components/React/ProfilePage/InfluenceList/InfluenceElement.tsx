import type { FC } from "react";
import type { Influence } from "src/libs/types/influence";
import EditableDescription from "../EditableDescription";
import InfluenceType from "./InfluenceType";
import MapCarousel from "@components/React/MapCarousel/SingleItem";
import BaseProfileCard from "@components/React/BaseProfileCard";

import styles from "./style.module.scss";

const InfluenceElement: FC<{
  influenceData: Influence;
  editable?: boolean;
}> = ({ influenceData, editable }) => {
  return (
    <>
      <div className={styles.influenceRow}>
        <div className={styles.cardWrapper}>
          <InfluenceType
            editable={editable}
            influenceType={influenceData.type}
          />
          <BaseProfileCard
            userData={influenceData.profileData}
            className={`${editable ? styles.editable : ""}`}
          />
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
