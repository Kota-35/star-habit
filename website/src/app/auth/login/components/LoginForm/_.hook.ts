import { useRouter } from "next/navigation";
import type React from "react";

export const useLoginForm = () => {
  const router = useRouter();

  const toSignupFormOnClick = (() => {
    router.push("/auth/signup");
  }) satisfies React.ComponentProps<"button">["onClick"];

  return { toSignupFormOnClick };
};
