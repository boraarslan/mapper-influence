import { AxiosError, AxiosResponse } from "axios";

export function mockAxios<T>(
  returnValue: T,
  timeout: number
): Promise<AxiosResponse<T>> {
  return new Promise((resolve) =>
    setTimeout(
      () => resolve({ data: returnValue, status: 200 } as AxiosResponse<T>),
      timeout
    )
  );
}

// Parameters are the same as above to quickly test for rejection
export function mockAxiosReject<T>(
  returnValue: T,
  timeout: number
): Promise<AxiosResponse<T>> {
  const error: Partial<AxiosError> = {
    isAxiosError: true,
    status: 401,
    message: "Unauthorized.",
  };

  throw error;
}
