import { usePathname, useRouter } from "next/navigation";
import { useEffect, useState } from "react";

type AuthGuardStatus = "pending" | "redirecting" | "allowed";

// useEffect 内でのみ呼ぶため、常にクライアント
const getStoredAccessToken = (): string | null =>
  localStorage.getItem("accessToken");

export const useAuthGuard = () => {
  const router = useRouter();
  const pathname = usePathname();
  const [status, setStatus] = useState<AuthGuardStatus>("pending");

  useEffect(() => {
    const token = getStoredAccessToken();
    if (!token) {
      setStatus("redirecting");
      router.replace(
        `/auth/login?redirect=${encodeURIComponent(pathname ?? "/")}`,
      );
      return;
    }
    setStatus("allowed");
  }, [router, pathname]);

  return { status };
};
