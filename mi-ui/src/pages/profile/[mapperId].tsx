import { NextPage } from "next";
import ProfilePage from "@components/PageComponents/ProfilePage";
import { userData } from "@libs/consts/dummyUserData";
import { useRouter } from "next/router";

const MapperPage: NextPage = () => {
  const router = useRouter();
  const { mapperId } = router.query;
  const numberId = parseInt(mapperId?.toString() || "");
  return (
    <>
      <ProfilePage userData={{ ...userData, id: numberId }} />
    </>
  );
};

export default MapperPage;
