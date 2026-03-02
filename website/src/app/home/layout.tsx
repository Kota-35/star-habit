import type React from "react";
import { AuthGuard } from "@/components/AuthGuard/_";

interface Props {
  children: React.ReactNode;
}

const HomeLayout = ({ children }: Props) => {
  return <AuthGuard>{children}</AuthGuard>;
};

export default HomeLayout;
