import { type FC, ReactNode, useState } from "react";
import SearchBar from "@components/React/Header/SearchBar";
import AddUserButton from "@components/React/ProfilePage/MapperDetails/AddUserButton";
import InfluenceType from "@components/React/ProfilePage/InfluenceList/InfluenceType";
import BaseProfileCard from "@components/React/BaseProfileCard";
import EditableDescription from "@components/React/ProfilePage/EditableDescription";
import { InfluenceTypeEnum, type Influence } from "src/libs/types/influence";
import Tooltip from "@components/React/Tooltip";

import styles from "./style.module.scss";

// TODO: Add featured map controls

const TutorialStep: FC<{
  number: number;
  title: string;
  children: ReactNode;
}> = ({ number, title, children }) => {
  return (
    <div className={styles.tutorialStep}>
      <h3>{`${number}. ${title}`}</h3>
      <div className={styles.centerer}>{children}</div>
    </div>
  );
};

type Props = { children?: ReactNode };
const TutorialScreen: FC<Props> = ({ children }) => {
  const [showTooltip, setShowTooltip] = useState(false);

  const toggleTooltip = () => {
    setShowTooltip(true);
    setTimeout(() => setShowTooltip(false), 3000);
  };

  return (
    <div className={styles.tutorialWrapper}>
      <h1>Getting Started:</h1>
      <div className={styles.stepsWrapper}>
        <TutorialStep
          number={1}
          title={"Look up someone who inspired your mapping"}
        >
          <div className={styles.searchWrapper}>
            <SearchBar />
          </div>
        </TutorialStep>
        <TutorialStep
          number={2}
          title={"Add the user to your influences list"}
        >
          <AddUserButton onClick={toggleTooltip} />
          {showTooltip && <Tooltip className="opacity-100" />}
        </TutorialStep>

        <TutorialStep
          number={3}
          title={"In your profile, describe how the mapper influenced you"}
        >
          <div className={styles.profileSide}>
            <InfluenceType editable />
            <BaseProfileCard
              userData={influenceData.profileData}
              className={styles.card}
            />
          </div>
          <div className={styles.descriptionSide}>
            <EditableDescription
              label="Description textarea in tutorial"
              description=""
              placeholder="Edit here to give more details."
              editable
            />
          </div>
        </TutorialStep>
      </div>
      {children}
    </div>
  );
};

const influenceData: Influence = {
  description: "Edit here to give details.",
  lastUpdated: new Date().getDate(),
  profileData: {
    avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
    username: "Fursum",
    id: 1234,
    flag: { code: "TR", name: "Türkiye" },
  },
  maps: [],
  strength: 1,
  type: InfluenceTypeEnum.Respect,
};

export default TutorialScreen;
