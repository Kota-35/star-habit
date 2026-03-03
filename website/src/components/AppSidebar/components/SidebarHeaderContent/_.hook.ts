import { PanelLeftClose, PanelLeftOpen } from "lucide-react";
import { useSidebar } from "@/components/ui/sidebar";

export const useSidebarHeaderContent = () => {
  const { state, toggleSidebar } = useSidebar();

  const isCollapsed = state === "collapsed";
  // 閉じているとき: ホバーで Star の代わりに「開く」アイコン。開いているとき: 「閉じる」アイコン。
  const ToggleIcon = isCollapsed ? PanelLeftOpen : PanelLeftClose;

  return {
    isCollapsed,
    ToggleIcon,
    toggleSidebar,
  };
};
