import { useRouter } from "next/navigation";
import type React from "react";
import { useState } from "react";
import type { DropdownMenuItem } from "@/components/ui/dropdown-menu";
import { useMe } from "@/server/__generated__/endpoints";
import { useSettingsDialog } from "./components/SettingsDialog";

export const useSidebarFooterContent = () => {
  const router = useRouter();

  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const handleLogoutOnClick = (() => {
    localStorage.removeItem("refreshToken");
    localStorage.removeItem("accessToken");

    router.push("/auth/login");
  }) satisfies React.ComponentProps<typeof DropdownMenuItem>["onClick"];

  const { data: userProfile } = useMe({
    query: {
      staleTime: 5 * 60 * 1000, // 5分
      refetchOnWindowFocus: false, // 頻繁に変更ないため
    },
  });

  const handleSettingsDialogOnSelect = ((event) => {
    event.preventDefault();
    setIsSettingsOpen(true);
  }) satisfies React.ComponentProps<typeof DropdownMenuItem>["onSelect"];

  return {
    handleLogoutOnClick,
    userProfile,
    handleSettingsDialogOnSelect,
    isSettingsOpen,
    setIsSettingsOpen,
  };
};
