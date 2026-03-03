"use client";

import type React from "react";
import { useAuthGuard } from "./_.hook";

interface Props {
  children: React.ReactNode;
}

export const AuthGuard = ({ children }: Props) => {
  const { status } = useAuthGuard();

  if (status !== "allowed") {
    return null;
  }

  return <>{children}</>;
};
