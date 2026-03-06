import clsx from "clsx";
import {
  type LucideIcon,
  Settings,
  SlidersHorizontal,
  User,
} from "lucide-react";
import type React from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Separator } from "@/components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface Props {
  open: boolean;
  onOpoenChange: React.ComponentProps<typeof Dialog>["onOpenChange"];
}

const tabs: {
  name: string;
  value: string;
  icon: LucideIcon;
  content: React.ReactNode;
}[] = [
  {
    name: "一般",
    value: "general",
    icon: SlidersHorizontal,
    content: <h1>General </h1>,
  },
  {
    name: "プロフィール",
    value: "profile",
    icon: User,
    content: <h1>Profile </h1>,
  },
];

export const SettingsDialog = (props: Props) => {
  const { open, onOpoenChange } = props;

  return (
    <Dialog open={open} onOpenChange={onOpoenChange}>
      <DialogContent
        className="sm:max-w-4xl"
        onOpenAutoFocus={(e) => e.preventDefault()} // ダイアログを表示した時の自動フォーカスを消す
      >
        <DialogHeader>
          <DialogTitle className="flex items-center text-xl">
            <Settings className="mr-3 h-5 w-5 text-blue-600" />{" "}
            <span className="text-gray-500">設定</span>
          </DialogTitle>
        </DialogHeader>
        <Separator className="-mx-4 bg-gray-200 shadow-sm data-[orientation=horizontal]:w-[calc(100%+2rem)]" />
        <Tabs
          orientation="vertical"
          defaultValue="general"
          className="min-h-72 items-stretch gap-0"
        >
          <TabsList className="h-full min-w-40 flex-col items-stretch justify-start gap-2 rounded-none bg-transparent p-0 pr-3">
            {tabs.map(({ name, value, icon: Icon }) => (
              <TabsTrigger
                key={value}
                value={value}
                className={clsx(
                  "h-auto flex-none items-center justify-start gap-1.5 px-2.5 sm:px-3",
                  "text-gray-500 hover:text-gray-700",
                  "data-[state=active]:bg-blue-50 data-[state=active]:text-blue-700 data-[state=active]:[&_svg]:text-blue-600",
                )}
              >
                <Icon /> {name}
              </TabsTrigger>
            ))}
          </TabsList>
          <Separator
            orientation="vertical"
            className="-my-4 bg-gray-200 shadow-sm"
          />

          {tabs.map(({ value, content }) => (
            <TabsContent key={value} value={value} className="pl-4">
              {content}
            </TabsContent>
          ))}
        </Tabs>
      </DialogContent>
    </Dialog>
  );
};
