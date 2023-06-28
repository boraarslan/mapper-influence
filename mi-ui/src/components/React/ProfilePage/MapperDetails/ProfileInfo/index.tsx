import { FC, useEffect } from "react";
import AddUserButton from "../AddUserButton";
import AwesomeDebouncePromise from "awesome-debounce-promise";
const textFit = require("textfit");

import styles from "./style.module.scss";
import type { UserBase } from "src/libs/types/user";
import { osuBaseUrl } from "src/libs/consts/urls";
import ProfilePhoto from "@components/React/ProfilePhoto";

type Props = {
  profileData: UserBase;
};

const ProfileInfo: FC<Props> = ({ profileData }) => {
  const ownProfile = false; // add path check

  const runFitText = () =>
    textFit(document.getElementsByClassName(styles.mapperName!));

  // Fit text to card on resize and on mount
  useEffect(() => {
    document.fonts.ready.then(() => runFitText());

    /*
    const debounceFitText = AwesomeDebouncePromise(
      runFitText,
      //Add random delay to updates
      50 + Math.random() * 15
    );
    */
    window.addEventListener("resize", runFitText());
    return () => {
      window.removeEventListener("resize", runFitText());
    };
  }, []);

  const renderGroup = () => {
    if (!profileData.groups?.length) return <></>;
    return (
      <div
        className={styles.title}
        style={{ color: profileData.groups[0]?.colour }}
      >
        {profileData.groups[0]?.name}
      </div>
    );
  };
  return (
    <div className={styles.profileInfo}>
      <a
        href={`${osuBaseUrl}profile/${profileData.id}`}
        target="_blank"
        rel="noreferrer"
      >
        <ProfilePhoto
          photoUrl={profileData.avatarUrl}
          size="xl"
          className={styles.avatar}
          circle
        />
      </a>
      <div className={styles.rightSide}>
        <a
          href={`${osuBaseUrl}profile/${profileData.id}`}
          target="_blank"
          rel="noreferrer"
        >
          <div className={styles.mapperName}>{profileData.username}</div>
        </a>
        {renderGroup()}
        {!ownProfile && (
          <AddUserButton
            onClick={() => {
              //TODO: Add service
            }}
          />
        )}
      </div>
    </div>
  );
};

export default ProfileInfo;
