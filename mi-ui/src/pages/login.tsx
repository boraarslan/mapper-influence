import { useRouter } from "next/router";
import { useEffect } from "react";
import { useCookies } from "react-cookie";
import { getUserBase } from "src/services/userBase";
import { useSessionStore } from "src/states/user";

//! THIS PAGE WILL NOT BE ROUTED TO IN PRODUCTION

const DevLoginPage = () => {
  const router = useRouter();
  const [_, setCookie] = useCookies(["mi-session-token"]);
  const { login } = useSessionStore();

  const session = router.query?.session || "testSession";

  useEffect(() => {
    setCookie("mi-session-token", session);
    getUserBase().then(({ data }) =>
      login(
        {
          avatarUrl: data.profile_picture,
          id: data.id,
          username: data.user_name,
          flag: { code: "TR", name: "Türkiye" },
        }
      )
    );
    //router.replace("/");
  }, [session, setCookie, login]);

  return <h1>{session}</h1>;
};
export default DevLoginPage;
