"use client";

import clsx from "clsx";
import { User } from "lucide-react";
import { SidebarFooter } from "@/components/ui/sidebar";

export const SidebarFooterContent = () => {
  return (
    <SidebarFooter className="mt-auto mb-4">
      <div
        className={clsx(
          "flex px-1",
          "group-data-[collapsible=icon]:justify-center",
        )}
      >
        <User className="h-5 w-5 shrink-0" />
      </div>
    </SidebarFooter>
  );
};
