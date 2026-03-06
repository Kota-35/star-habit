"use client";

import { Home, type LucideIcon, NotebookPen } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

const SIDEBAR_MENU_ITEMS: { path: string; label: string; icon: LucideIcon }[] =
  [
    { path: "/home", label: "ホーム", icon: Home },
    { path: "/star", label: "STARログ作成", icon: NotebookPen },
  ];

export const SidebarBodyContent = () => {
  const pathname = usePathname();

  return (
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupContent>
          <SidebarMenu className="flex flex-col">
            {SIDEBAR_MENU_ITEMS.map((item) => {
              const Icon = item.icon;
              const isActive = pathname === item.path;

              return (
                <SidebarMenuItem key={item.path}>
                  <SidebarMenuButton
                    asChild
                    isActive={isActive}
                    tooltip={item.label}
                    className="select-none rounded-2xl py-1 text-gray-500 hover:bg-blue-100/20 hover:text-gray-700 data-active:bg-blue-50 data-active:text-blue-700 data-active:[&_svg]:text-blue-600"
                  >
                    <Link href={item.path}>
                      <Icon />
                      <span>{item.label}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              );
            })}
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
  );
};
