import { createFileRoute, Navigate } from "@tanstack/react-router";
import { useCurrentUser } from "@/hooks/useCurrentUser";

export const Route = createFileRoute("/")({
  component: RootNavigate,
});

function RootNavigate() {
  const { user, isLoading, hasToken } = useCurrentUser();

  // No token in localStorage, go to login
  if (!hasToken) {
    return <Navigate to="/login" replace />;
  }

  // Token exists but still loading user data, wait
  if (isLoading) {
    return null; // or a loading spinner
  }

  // Token exists and loading complete - check if user data is valid
  if (!user) {
    return <Navigate to="/login" replace />;
  }

  return <Navigate to="/search" replace />;
}
