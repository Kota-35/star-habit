import type React from "react";
import { useState } from "react";
import type { PasswordField } from "./_";

type Props = React.ComponentProps<typeof PasswordField>;

export const usePasswordField = (props: Props) => {
  const [visible, setVisible] = useState(false);

  return { visible, setVisible, ...props };
};
