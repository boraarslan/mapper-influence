import axios from "axios";
import { mockRequest, mockAxiosReject } from "@libs/functions";
import { useCurrentUser } from "@hooks/useUser";
import { useQuery } from "@tanstack/react-query";
import { DUMMY_INFLUENCES } from "@libs/consts/dummyUserData";

export type InfluenceResponse = {
  from_id: number;
  to_id: number;
  influence_level: number;
  info?: string;
  created_at: any;
  modified_at: any;
};

export async function getInfluences(userId: string | number) {
  if (process.env.NODE_ENV !== "production")
    return mockRequest(DUMMY_INFLUENCES, 1000);

  let searchUrl = "/api/v1/influence/get/" + userId;
  return axios.get<InfluenceResponse[]>(searchUrl).then((res) => res.data);
}

export const useGetInfluences = (userId?: string | number) => {
  const { user } = useCurrentUser();
  return useQuery({
    queryKey: ["influences", userId],
    queryFn: () => getInfluences(userId || user?.id || 0),
    staleTime: 60 * 1000,
  });
};

export type AddInfluenceRequest = {
  from_id: number;
  level: number;
  info?: string;
};

export async function addInfluence(body: AddInfluenceRequest) {
  // Mock data for dev
  if (process.env.NODE_ENV !== "production") return mockRequest({}, 1000);

  let searchUrl = "/api/v1/influence/create/";
  return await axios.post(searchUrl, body);
}

export async function deleteInfluence(from_id: string | number) {
  // Mock data for dev
  if (process.env.NODE_ENV !== "production") return mockRequest({}, 1000);

  let searchUrl = "/api/v1/influence/delete/";
  return await axios.post(searchUrl, { from_id });
}

export type EditInfluenceInfoRequest = {
  from_id: number;
  info?: string;
};

export async function editInfluenceInfo(body: EditInfluenceInfoRequest) {
  // Mock data for dev
  if (process.env.NODE_ENV !== "production") return mockRequest({}, 1000);

  let searchUrl = "/api/v1/influence/update/info";
  return await axios.post(searchUrl, body);
}

export type EditInfluenceLevelRequest = {
  from_id: number;
  level: number;
};

export async function editInfluenceLevel(body: EditInfluenceLevelRequest) {
  // Mock data for dev
  if (process.env.NODE_ENV !== "production") return mockAxiosReject({}, 1000);

  let searchUrl = "/api/v1/influence/update/level";
  return await axios.post(searchUrl, body);
}
