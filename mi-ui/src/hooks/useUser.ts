import { getUserBase } from "@services/userBase";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { useCookies } from "react-cookie";
import { useSessionStore } from "src/states/user";

export const useUser = () => {
  const router = useRouter();
  const { user, login, logout } = useSessionStore();
  const [cookie, _, deleteCookie] = useCookies(["mi-session-token"]);

  const sessionToken = cookie["mi-session-token"];

  useEffect(() => {
    if (!user && sessionToken)
      getUserBase().then(({ data }) =>
        login({
          avatarUrl: data.profile_picture,
          id: data.id,
          username: data.user_name,
        })
      );
    if (!sessionToken) {
      logout();
      if (router.pathname !== "/") router.push("/");
    }
  }, [sessionToken, user, login, logout, router]);

  return {
    user,
    logout: () => {
      logout();
      deleteCookie("mi-session-token");
    },
  };
};
