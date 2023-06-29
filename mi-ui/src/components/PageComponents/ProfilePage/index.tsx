import { FC, useMemo, useState } from "react";
import { useRouter } from "next/router";
import { UserProfile, UserBase } from "@libs/types/user";
import InfluenceList from "./InfluenceList";
import MapperDetails from "./MapperDetails";
import MentionList from "./MentionList";
import { useUser } from "@hooks/useUser";

import styles from "./style.module.scss";

type Props = { userData: UserProfile; editable?: boolean };

const ProfilePage: FC<Props> = ({ userData, editable = false }) => {
  const { logout } = useUser();
  const router = useRouter();
  const [selectedTab, setSelectedTab] = useState<"influences" | "mentions">(
    "influences"
  );

  const isUser = router.asPath === "/profile";

  const InfluenceTab = useMemo(() => {
    switch (selectedTab) {
      case "influences":
        return (
          <InfluenceList influences={userData.influences} editable={editable} />
        );
      case "mentions":
        return <MentionList mentions={userData.mentions} />;
      default:
        return <></>;
    }
  }, [selectedTab, userData, editable]);

  return (
    <div className={styles.profilePage}>
      <MapperDetails
        description={userData.description}
        mapList={userData.maps}
        details={userData.details}
        profileData={userData as UserBase}
        editable={editable}
      />

      <div className={styles.buttons}>
        <button
          className={selectedTab === "influences" ? styles.selected : ""}
          onClick={() => setSelectedTab("influences")}
        >
          Influences
        </button>
        <button
          className={selectedTab === "mentions" ? styles.selected : ""}
          onClick={() => setSelectedTab("mentions")}
        >
          Mentions
        </button>
      </div>
      <div className={styles.content}>{InfluenceTab}</div>

      {isUser && <button onClick={logout}>Sign out</button>}
    </div>
  );
};

export default ProfilePage;
