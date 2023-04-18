import { NextPage } from "next";
import { useRouter } from "next/router";
import { useCallback } from "react";
import ProfilePage from "@components/PageComponents/ProfilePage";
import { userData } from "@libs/consts/dummyUserData";
import { getUserBase } from "src/services/userBase";
import { useFetchService } from "@hooks/useFetchService";

const MapperPage: NextPage = () => {
  const router = useRouter();
  const { mapperId } = router.query;
  const fetchUser = useCallback(
    () => getUserBase(mapperId?.toString()),
    [mapperId]
  );
  const [user, error, loading] = useFetchService(fetchUser);

  let mergedData = userData;
  if (user)
    mergedData = {
      ...mergedData,
      avatarUrl: user.profile_picture,
      username: user.user_name,
      id: user.id,
    };

  if (error) return <h1>Error while fetching user: {error}</h1>;
  if (loading) return <span>Loading...</span>;
  if (user) return <ProfilePage userData={mergedData} />;
  return <h1>User not found! {error}</h1>;
};

export default MapperPage;
