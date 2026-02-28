import type { AxiosRequestConfig } from "axios";
import Axios from "axios";
import { env } from "@/lib/env";

/**
 * Axiosの custom instance を設定する
 *
 *  @see https://www.orval.dev/guides/custom-axios
 */

export const AXIOS_INSTANCE = Axios.create({
  baseURL: env.NEXT_PUBLIC_SERVER_ORIGIN,
});

export const customInstance = <T>(
  config: AxiosRequestConfig,
  options?: AxiosRequestConfig,
): Promise<T> => {
  return AXIOS_INSTANCE({
    ...config,
    ...options,
  }).then(({ data }) => data);
};

export type ErrorType<Error> = import("axios").AxiosError<Error>;
export type BodyType<BodyData> = BodyData;
