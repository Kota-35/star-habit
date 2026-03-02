import { usePathname, useRouter } from "next/navigation";
import { useCallback, useEffect } from "react";

export const useAuthGuard = () => {
  const router = useRouter();
  const pathname = usePathname();

  const getAccessToken = useCallback(() => {
    if (typeof window === "undefined") return null;
    return localStorage.getItem("accessToken");
  }, []);

  useEffect(() => {
    const token = getAccessToken();
    if (!token) {
      router.replace(
        `/auth/login?redirect=${encodeURIComponent(pathname ?? "/")}`,
      );

      return;
    }
  }, [router, pathname, getAccessToken]);

  return { getAccessToken };
};
