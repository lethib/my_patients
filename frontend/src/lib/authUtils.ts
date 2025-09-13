import { router } from "@/App";
import { queryClient } from "@/api/api";

export const logout = () => {
  localStorage.removeItem("accessToken");
  queryClient.clear();
  router.navigate({ to: "/" });
};
