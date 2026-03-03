"use client";

import type React from "react";
import { Sidebar, SidebarInset, SidebarProvider } from "../ui/sidebar";
import { SidebarBodyContent } from "./components/SidebarBodyContent";
import { SidebarFooterContent } from "./components/SidebarFooterContent";
import { SidebarHeaderContent } from "./components/SidebarHeaderContent";

interface Props {
  children: React.ReactNode;
}

export const AppSidebar = ({ children }: Props) => {
  return (
    <SidebarProvider>
      <AppSidebarContent>{children}</AppSidebarContent>
    </SidebarProvider>
  );
};

const AppSidebarContent = ({ children }: Props) => {
  return (
    <>
      <Sidebar collapsible="icon" className="border-gray-200 shadow-sm">
        <SidebarHeaderContent />
        <SidebarBodyContent />
        <SidebarFooterContent />
      </Sidebar>

      <SidebarInset className={"flex min-h-svh flex-col"}>
        {children}
      </SidebarInset>
    </>
  );
};
