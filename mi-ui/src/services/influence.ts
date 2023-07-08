import axios from "axios";
import { mockRequest, mockAxiosReject } from "@libs/functions";
import { useCurrentUser } from "@hooks/useUser";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { DUMMY_INFLUENCES } from "@libs/consts/dummyUserData";
import { UserFullResponse } from "./user";
import { toast } from "react-toastify";

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

  let searchUrl = "/api/v1/influence/get" + `/${userId}`;
  return axios.get<InfluenceResponse[]>(searchUrl).then((res) => res.data);
}

export const useGetInfluences = (userId?: string | number) => {
  const { user } = useCurrentUser();
  const id = userId || user?.id || 0;
  return useQuery({
    queryKey: ["influences", id],
    queryFn: () => getInfluences(id),
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

  let searchUrl = "/api/v1/influence/create";
  return await axios.post(searchUrl, body);
}

export const useAddInfluenceMutation = () => {
  const queryClient = useQueryClient();
  const { user } = useCurrentUser();
  const key = ["influences", user?.id];

  return useMutation({
    mutationFn: addInfluence,
    onSuccess: (_, variables) => {
      queryClient.cancelQueries(key);
      queryClient.setQueryData(key, (old: InfluenceResponse[] | undefined) => {
        const newInfluence = {
          from_id: user?.id || 0,
          to_id: variables.from_id,
          influence_level: variables.level,
          info: variables.info,
          created_at: new Date(),
          modified_at: new Date(),
        };
        if (!old) return [newInfluence];
        return [...old, newInfluence];
      });
      toast.success("Influence added successfully.");
    },
    onError: () => toast.error("Failed to add influence."),
    onSettled: () => queryClient.invalidateQueries(key),
  });
};

export async function deleteInfluence(from_id: string | number) {
  // Mock data for dev
  if (process.env.NODE_ENV !== "production") return mockRequest({}, 1000);

  let searchUrl = `/api/v1/influence/delete/${from_id}`;
  return await axios.delete(searchUrl);
}

export const useDeleteInfluenceMutation = () => {
  const queryClient = useQueryClient();
  const { user } = useCurrentUser();
  const key = ["influences", user?.id];

  return useMutation({
    mutationFn: deleteInfluence,
    onSuccess: (_, variables) => {
      queryClient.cancelQueries(key);
      queryClient.setQueryData(key, (old: InfluenceResponse[] | undefined) => {
        if (!old) return [];
        return old.filter((influence) => influence.from_id !== variables);
      });
      toast.success("Influence removed successfully.");
    },
    onError: () => toast.error("Failed to remove influence."),
    onSettled: () => queryClient.invalidateQueries(key),
  });
};

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
