import { APIClient } from "@/api/api";

export const useCurrentUser = () => {
  const STALE_TIME = 3 * 60 * 1_000; // 3 minutes
  const accessToken = localStorage.getItem("accessToken");
  const currentUserQuery = APIClient.hooks.auth.me.useQuery(null, {
    enabled: !!accessToken,
    staleTime: STALE_TIME,
  });

  return {
    currentUser: currentUserQuery.data,
    isLoading: currentUserQuery.isLoading,
    hasToken: !!accessToken,
  };
};
