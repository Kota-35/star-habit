import type React from "react";

interface Props {
  children: React.ReactNode;
}

const SignupLayout = ({ children }: Props) => {
  return <div className="min-h-screen">{children}</div>;
};

export default SignupLayout;
