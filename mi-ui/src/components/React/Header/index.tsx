import type { FC } from "react";
import type { UserBase } from "src/libs/types/user";
import DarkModeToggle from "../DarkMode/ThemeToggle";
import styles from "./style.module.scss";
import Influences from "../Svg/Influences";
import SearchBar from "./SearchBar";
import ProfilePhoto from "../ProfilePhoto";
import { useUser } from "src/hooks/user";

export default function Header() {
  const user = useUser();

  if (!user) return <></>;

  return (
    <div className={styles.header}>
      <a
        href="/"
        className={styles.home}
      >
        <Influences />
        <span>Mapper Influences</span>
      </a>
      <SearchBar className={styles.searchBar} />
      <DarkModeToggle className={styles.darkMode} />
      <ProfileLinkAvatar user={user} />
    </div>
  );
}

const ProfileLinkAvatar: FC<{ user?: UserBase }> = ({ user }) => (
  <a href={"/profile"}>
    <ProfilePhoto
      className={styles.avatar}
      photoUrl={user?.avatarUrl}
      size="md"
      circle
    />
  </a>
);
