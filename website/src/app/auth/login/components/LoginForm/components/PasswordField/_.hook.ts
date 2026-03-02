import type React from "react";
import { useState } from "react";
import type { PasswordField } from "./_";

type Props = React.ComponentProps<typeof PasswordField>;

export const userPasswordField = (props: Props) => {
  const [showPassword, setShowPassword] = useState(false);

  const onTogglePasswordVisibility = (() => {
    setShowPassword(!showPassword);
  }) satisfies React.ComponentProps<"button">["onClick"];

  return {
    showPassword,
    onTogglePasswordVisibility,

    ...props,
  };
};
