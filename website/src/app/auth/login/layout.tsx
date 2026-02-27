import type React from "react";

interface Props {
  children: React.ReactNode;
}

const LoginLayout = ({ children }: Props) => {
  return <div className="min-h-screen">{children}</div>;
};

export default LoginLayout;
