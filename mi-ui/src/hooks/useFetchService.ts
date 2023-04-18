import { useEffect, useState } from "react";
import useUnauthorized from "./useUnauthorized";
import { AxiosResponse, isAxiosError } from "axios";

export const useFetchService = <T>(
  fetcher: () => Promise<AxiosResponse<T>>
) => {
  const [data, setData] = useState<T>();
  const [error, setError] = useState<string>();
  const [loading, setLoading] = useState<boolean>();
  
  const { redirect } = useUnauthorized();

  useEffect(() => {
    setLoading(true);
    fetcher()
      .then(({ data }) => setData(data))
      .catch((err) => {
        if (!isAxiosError(err)) {
          setError(err.toString());
          return;
        }
        if (err.status === 401) redirect();
        setError(err.message);
      })
      .finally(() => setLoading(false));
  }, [fetcher, redirect]);

  return [data, error, loading] as const;
};
