"use client";

import clsx from "clsx";
import { Star } from "lucide-react";
import { match } from "ts-pattern";
import { Button } from "@/components/ui/button";
import { SidebarHeader } from "@/components/ui/sidebar";
import { useSidebarHeaderContent } from "./_.hook";

export const SidebarHeaderContent = () => {
  const { isCollapsed, toggleSidebar, ToggleIcon } = useSidebarHeaderContent();

  return (
    <SidebarHeader>
      <div
        className={clsx(
          "group/header relative flex items-center justify-between px-1",
          "group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:px-0",
        )}
      >
        {match(isCollapsed)
          .with(true, () => (
            <div
              className={clsx(
                "relative flex size-8 items-center justify-center",
              )}
            >
              <Star
                className={clsx(
                  "h-5 w-5 shrink-0 transition-opacity",
                  "group-data-[collapsible=icon]:group-hover/header:opacity-0",
                )}
              />
              <Button
                onClick={toggleSidebar}
                size="icon"
                className={clsx(
                  "size-8 shrink-0 transition-opacity",
                  "absolute inset-0 opacity-0 group-hover/header:opacity-100",
                )}
                aria-label="サイドバーを開く"
              >
                <ToggleIcon className="h-5 w-5 shrink-0" />
              </Button>
            </div>
          ))
          .otherwise(() => (
            <>
              <Star className="h-5 w-5 shrink-0" />
              <Button
                onClick={toggleSidebar}
                size="icon"
                className={clsx("size-8 shrink-0")}
                aria-label="サイドバーを閉じる"
              >
                <ToggleIcon className="h-5 w-5" />
              </Button>
            </>
          ))}
      </div>
    </SidebarHeader>
  );
};
