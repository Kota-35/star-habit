import type React from "react";
import { AppSidebar } from "@/components/AppSidebar";
import { AuthGuard } from "@/components/AuthGuard/_";
import { Header } from "@/components/Header";

interface Props {
  children: React.ReactNode;
}

const HomeLayout = ({ children }: Props) => {
  return (
    <AuthGuard>
      <AppSidebar>
        <Header />
        <div className="min-h-0 flex-1 bg-gray-100">{children}</div>
      </AppSidebar>
    </AuthGuard>
  );
};

export default HomeLayout;
