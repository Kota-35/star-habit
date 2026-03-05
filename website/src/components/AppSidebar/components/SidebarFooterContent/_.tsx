"use client";

import clsx from "clsx";
import { ChevronsUpDown, LogOut, Settings, User } from "lucide-react";
import { match, P } from "ts-pattern";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  SidebarFooter,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { Skeleton } from "@/components/ui/skeleton";
import { useSidebarFooterContent } from "./_.hook";
import { SettingsDialog } from "./components/SettingsDialog";

export const SidebarFooterContent = () => {
  const {
    handleLogoutOnClick,
    userProfile,
    handleSettingsDialogOnSelect,
    isSettingsOpen,
    setIsSettingsOpen,
  } = useSidebarFooterContent();

  return (
    <SidebarFooter className="mt-auto mb-4">
      <div
        className={clsx("flex", "group-data-[collapsible=icon]:justify-center")}
      >
        <SidebarMenu>
          <SidebarMenuItem>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <SidebarMenuButton className="h-auto select-none rounded-2xl py-2 hover:bg-blue-100/20 data-[state=closed]:focus-visible:ring-0">
                  <div className="flex w-full items-center gap-2 px-1">
                    <User className="h-7 w-7 shrink-0" />

                    <div className="flex min-w-0 flex-col gap-1">
                      {match(userProfile)
                        .with(P.nullish, () => (
                          <>
                            <Skeleton className="h-6 w-24 rounded-md bg-gray-200 opacity-70" />
                            <Skeleton className="h-5 w-40 rounded-md bg-gray-200 opacity-70" />
                          </>
                        ))
                        .otherwise(({ username, email }) => (
                          <>
                            <p className="text-xl">{username}</p>
                            <p className="text-xs">{email}</p>
                          </>
                        ))}
                    </div>

                    <ChevronsUpDown className="ml-auto h-7 w-7 shrink-0" />
                  </div>
                </SidebarMenuButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                side="right"
                align="start"
                sideOffset={10}
                className="w-[--radix-popper-anchor-width]"
              >
                <DropdownMenuItem onSelect={handleSettingsDialogOnSelect}>
                  <Settings className="h5 w-5" />
                  <span>設定</span>
                </DropdownMenuItem>
                <SettingsDialog
                  open={isSettingsOpen}
                  onOpoenChange={setIsSettingsOpen}
                />
                <DropdownMenuSeparator className="bg-gray-200" />
                <DropdownMenuItem
                  variant="destructive"
                  onClick={handleLogoutOnClick}
                >
                  <LogOut className="h-5 w-5" />
                  <span>ログアウト</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </SidebarMenuItem>
        </SidebarMenu>
      </div>
    </SidebarFooter>
  );
};
