import { useEffect, useState } from "react";
import axios from "axios";
import { userData } from "@libs/consts/dummyUserData";
import { timeoutValue } from "@libs/functions";
import { UserBase } from "@libs/types/user";

type ServiceReturn = {
  id: number;
  user_name: string;
  profile_picture: string;
  bio?: string;
};

export async function getUserBase(userId?: string): Promise<UserBase> {
  // Mock data for dev
  if (process.env.NODE_ENV !== "production")
    return timeoutValue<UserBase>(userData, 200);

  const constructedUrl = new URL("/api/v1/user/get");
  if (userId) constructedUrl.searchParams.set("user_id", userId);

  const { data } = await axios.get<ServiceReturn>(constructedUrl.toString());
  return {
    avatarUrl: data.profile_picture,
    id: data.id,
    username: data.user_name,
  };
}

export const useGetUserBase = (userId?: string) => {
  const [user, setUser] = useState<UserBase>();
  const [error, setError] = useState<string>();
  const [loading, setLoading] = useState<boolean>();

  useEffect(() => {
    setLoading(true);
    getUserBase(userId)
      .then((data) => setUser(data))
      .catch((err) => setError(err.toString()))
      .finally(() => setLoading(false));
  }, [userId]);

  return { user, error, loading };
};
