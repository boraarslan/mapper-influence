import { NextPage } from "next";
import ProfilePage from "@components/PageComponents/ProfilePage";
import { UserProfile } from "@libs/types/user";
import { useUser } from "@hooks/useUser";

const MapperPage: NextPage = () => {
  const { user } = useUser();

  let mergedData = dummyData;
  if (user)
    mergedData = {
      ...dummyData,
      ...user,
    };

  //if (error) return <h1>Error while fetching user: {error}</h1>;
  //if (loading) return <span>Loading...</span>;
  if (user) return <ProfilePage userData={mergedData} editable />;
  //return <h1>User not found! {error}</h1>;
};

export default MapperPage;

const dummyData: UserProfile = {
  description: "",
  details: {
    followerCount: 1,
    graveyardCount: 1,
    lovedCount: 0,
    pendingCount: 1,
    rankedCount: 0,
    subCount: 2,
  },
  id: 12345,
  influences: [],
  maps: [],
  mentions: [],
  username: "Skytuna",
  avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
  flag: { code: "TR", name: "Türkiye" },
};
