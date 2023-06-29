import { FC, useEffect, useMemo, useRef } from "react";
import { useRouter } from "next/router";
import ProfilePhoto from "@components/SharedComponents/ProfilePhoto";
import { osuBaseUrl } from "@libs/consts/urls";
import { UserBase } from "@libs/types/user";
import AddUserButton from "../AddUserButton";
import AwesomeDebouncePromise from "awesome-debounce-promise";
const textFit = require("textfit");

import styles from "./style.module.scss";

type Props = {
  profileData: UserBase;
};

const ProfileInfo: FC<Props> = ({ profileData }) => {
  const router = useRouter();
  const ownProfile = useMemo(() => {
    return router.asPath === "/profile";
  }, [router]);

  const nameRef = useRef(null);

  // Fit text to card on resize and on mount
  useEffect(() => {
    if (!nameRef.current) return;
    const runFitText = () => textFit(nameRef.current);
    const debounceFitText = AwesomeDebouncePromise(runFitText, 50);

    document.fonts.ready.then(() => runFitText());
    window.addEventListener("resize", debounceFitText);
    return () => {
      window.removeEventListener("resize", debounceFitText);
    };
  }, [nameRef]);

  const renderGroup = () => {
    if (!profileData.groups?.length) return <></>;
    return (
      <div
        className={styles.title}
        style={{ color: profileData.groups[0].colour }}
      >
        {profileData.groups[0].name}
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
          <div className={styles.mapperName} ref={nameRef}>
            {profileData.username}
          </div>
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
