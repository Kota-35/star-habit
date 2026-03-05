import type React from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

interface Props {
  open: boolean;
  onOpoenChange: React.ComponentProps<typeof Dialog>["onOpenChange"];
}

export const SettingsDialog = (props: Props) => {
  const { open, onOpoenChange } = props;

  return (
    <Dialog open={open} onOpenChange={onOpoenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>設定</DialogTitle>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
};
