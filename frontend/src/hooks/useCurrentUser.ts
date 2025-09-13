import { APIClient } from "@/api/api";

export const useCurrentUser = () => {
  const accessToken = localStorage.getItem("accessToken");
  const currentUserQuery = APIClient.hooks.auth.me.useQuery(null, {
    enabled: !!accessToken,
  });

  return {
    currentUser: currentUserQuery.data,
    isLoading: currentUserQuery.isLoading,
    hasToken: !!accessToken,
  };
};
