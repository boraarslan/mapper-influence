import { NextPage } from "next";
import ProfilePage from "@components/PageComponents/ProfilePage";
import { userData } from "@libs/consts/dummyUserData";
import { useRouter } from "next/router";
import { useGetUserBase } from "src/services/userBase";

const MapperPage: NextPage = () => {
  const router = useRouter();
  const { mapperId } = router.query;
  const { user, error, loading } = useGetUserBase(mapperId?.toString());

  let mergedData = userData;
  if (user)
    mergedData = {
      ...mergedData,
      ...user,
    };

  if (error) return <h1>Error while fetching user: {error}</h1>;
  return (
    <>
      <ProfilePage userData={mergedData} />
    </>
  );
};

export default MapperPage;
