import { createFileRoute, Navigate } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: RootNavigate,
});

function RootNavigate() {
  if (!localStorage.getItem("accessToken")) {
    return <Navigate to="/login" replace />;
  }

  return <Navigate to="/search" replace />;
}
