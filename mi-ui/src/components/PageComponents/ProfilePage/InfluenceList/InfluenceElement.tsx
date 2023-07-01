import { FC } from "react";
import BaseProfileCard from "@components/SharedComponents/BaseProfileCard";
import MapCarousel from "@components/SharedComponents/MapCarousel/SingleItem";
import { convertToInfluence } from "@libs/enums";
import { InfluenceResponse } from "@services/influence";
import InfluenceType from "./InfluenceType";
import EditableDescription from "../EditableDescription";

import styles from "./style.module.scss";

const InfluenceElement: FC<{
  influenceData: InfluenceResponse;
  editable?: boolean;
}> = ({ influenceData, editable }) => {
  return (
    <>
      <div className={styles.influenceRow}>
        <div className={styles.cardWrapper}>
          <InfluenceType
            editable={editable}
            influenceType={convertToInfluence(influenceData.influence_level)}
          />
          <BaseProfileCard
            userId={influenceData.from_id}
            className={`${editable ? styles.editable : ""}`}
          />
        </div>
        <EditableDescription
          className={styles.description}
          label={`Description textarea`}
          description={influenceData.info || ""}
          editable={editable}
          placeholder={"Describe your influence here."}
        />
        {false && (
          <div className={styles.maps}>
            <h4>Featured Maps</h4>
            <MapCarousel mapList={[]} />
          </div>
        )}
      </div>
    </>
  );
};

export default InfluenceElement;
