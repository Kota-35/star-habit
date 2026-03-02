import type { AxiosRequestConfig } from "axios";
import Axios from "axios";
import { env } from "@/lib/env";

const getAccessToken = (): string | undefined => {
  if (typeof window === "undefined") return undefined;
  return localStorage.getItem("accessToken") ?? undefined;
};

const getRefreshToken = (): string | undefined => {
  if (typeof window === "undefined") return undefined;
  return localStorage.getItem("refreshToken") ?? undefined;
};

const setTokens = (accessToken: string, refreshToken: string) => {
  if (typeof window === "undefined") return;
  localStorage.setItem("accessToken", accessToken);
  localStorage.setItem("refreshToken", refreshToken);
};

const clearTokensAndRedirectToLogin = () => {
  if (typeof window === "undefined") return;
  localStorage.removeItem("accessToken");
  localStorage.removeItem("refreshToken");
  window.location.href = "/auth/login";
};

/** refresh の同時実行を防ぐため、進行中ならその Promise を共有する */
let refreshPromise: Promise<{
  accessToken: string;
  refreshToken: string;
} | null> | null = null;

const callRefreshApi = async (): Promise<{
  accessToken: string;
  refreshToken: string;
} | null> => {
  const refreshToken = getRefreshToken();
  if (!refreshToken) return null;

  // refresh は ボディの refreshToken だけで認証
  // AXIOS_INSTANCE を使うと request インターセプターで 常に Authorization: Bearer <localStorage の accessToken> が付くので、
  // fetchを使用
  const baseURL = env.NEXT_PUBLIC_SERVER_ORIGIN;
  const res = await fetch(`${baseURL}/api/auth/refresh`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ refreshToken }),
  });

  if (!res.ok) return null;
  const data = (await res.json()) as {
    accessToken: string;
    refreshToken: string;
  };
  return { accessToken: data.accessToken, refreshToken: data.refreshToken };
};

const doRefresh = async (): Promise<{
  accessToken: string;
  refreshToken: string;
} | null> => {
  if (refreshPromise) return refreshPromise;
  refreshPromise = callRefreshApi();
  try {
    const result = await refreshPromise;
    return result;
  } finally {
    refreshPromise = null;
  }
};

/**
 * Axiosの custom instance を設定する
 *
 *  @see https://www.orval.dev/guides/custom-axios
 */

export const AXIOS_INSTANCE = Axios.create({
  baseURL: env.NEXT_PUBLIC_SERVER_ORIGIN,
});

// リクエスト送信前に Bearer を付与
AXIOS_INSTANCE.interceptors.request.use((config) => {
  const token = getAccessToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// 401 時に refresh して再送する
AXIOS_INSTANCE.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config as AxiosRequestConfig & {
      _retry?: boolean;
    };

    if (error.response?.status !== 401 || originalRequest._retry) {
      return Promise.reject(error);
    }

    const refreshed = await doRefresh();
    if (!refreshed) {
      clearTokensAndRedirectToLogin();
      return Promise.reject(error);
    }

    setTokens(refreshed.accessToken, refreshed.refreshToken);
    originalRequest._retry = true;
    originalRequest.headers = originalRequest.headers ?? {};
    originalRequest.headers.Authorization = `Bearer ${refreshed.accessToken}`;

    return AXIOS_INSTANCE(originalRequest);
  },
);

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
