import { FC } from "react";
import { toast } from "react-toastify";
import BaseProfileCard from "@components/SharedComponents/BaseProfileCard";
import MapCarousel from "@components/SharedComponents/MapCarousel/SingleItem";
import { convertFromInfluence, convertToInfluence } from "@libs/enums";
import {
  InfluenceResponse,
  editInfluenceInfo,
  editInfluenceLevel,
} from "@services/influence";
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
            influenceData={influenceData}
            onChange={(type) =>
              editInfluenceLevel({
                from_id: influenceData.from_id,
                level: convertFromInfluence(type),
              })
            }
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
          onChange={(e) =>
            editInfluenceInfo({
              from_id: influenceData.from_id,
              info: e.target.value,
            })
          }
          statusText={{
            loading: "Submitting influence description.",
            error: "Could not update influence description.",
            success: "Updated influence description.",
          }}
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
