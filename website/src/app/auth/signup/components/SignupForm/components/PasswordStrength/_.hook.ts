import type React from "react";
import type { PasswordStrength } from "./_";

type Props = React.ComponentProps<typeof PasswordStrength>;

export const usePasswordStrength = (props: Props) => {
  const { password, isVisible } = props;

  const shouldShowStrength = isVisible;

  const isPasswordAtLeast8Chars = password.length >= 8;
  const isPasswordContainingDigit = /\d/.test(password);

  return {
    isPasswordAtLeast8Chars,
    isPasswordContainingDigit,

    shouldShowStrength,
  };
};
