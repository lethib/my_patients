import { router } from "@/App";
import { queryClient } from "@/api/api";

export const logout = () => {
  localStorage.removeItem("accessToken");
  queryClient.clear();
  router.navigate({ to: "/" });
};

export const login = (token: string) => {
  localStorage.setItem("accessToken", token);
  router.navigate({ to: "/", replace: true });
};
