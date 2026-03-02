"use client";

import type React from "react";
import { useAuthGuard } from "./_.hook";

interface Props {
  children: React.ReactNode;
}

export const AuthGuard = ({ children }: Props) => {
  const { getAccessToken } = useAuthGuard();

  // 初回レンダー時はまだ localStorage を読めない（SSR）なので、チェック中は何も出さない or ローダー
  if (typeof window === "undefined") {
    return null; // または <LoadingSpinner />
  }
  if (!getAccessToken()) {
    return null; // リダイレクトするまで何も表示しない（またはローダー）
  }

  return <>{children}</>;
};
