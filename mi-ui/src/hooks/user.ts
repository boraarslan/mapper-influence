import { useStore } from "@nanostores/react";
import { useEffect, useState } from "react";
import type { UserBase } from "src/libs/types/user";
import { getUserBase } from "src/services/userBase";
import { userStore } from "src/states/user";

export const useUser = () => {
  const userFromStore = useStore(userStore);
  const [user, setUser] = useState<UserBase | null>(userFromStore);

  console.log("rendere")

  useEffect(() => {
    if (!user && userFromStore) setUser(userFromStore);

    if (!userFromStore)
      getUserBase().then(({ data }) => {
        console.log("fetched")
        const userBase: UserBase = {
          id: data.id,
          username: data.user_name,
          avatarUrl: data.profile_picture,
        };
        setUser(userBase);
        userStore.set(userBase);
      });
  }, [userFromStore]);

  return user;
};
