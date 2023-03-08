import axios from "axios";
import { userData } from "@libs/consts/dummyUserData";
import { mockAxios } from "@libs/functions";

type ServiceReturn = {
  id: number;
  user_name: string;
  profile_picture: string;
  bio?: string;
};

export async function getUserBase(userId?: string) {
  // Mock data for dev
  if (process.env.NODE_ENV !== "production")
    return mockAxios<ServiceReturn>(
      {
        id: userData.id,
        profile_picture: userData.avatarUrl,
        user_name: userData.username,
      },
      1000
    );

  const constructedUrl = new URL("/api/v1/user/get");
  if (userId) constructedUrl.searchParams.set("user_id", userId);

  return await axios.get<ServiceReturn>(constructedUrl.toString());
}
