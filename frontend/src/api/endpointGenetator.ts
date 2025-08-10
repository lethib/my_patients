import { useMutation, useQuery } from "@tanstack/react-query";
import { APIClient } from "./api";

// Base endpoint configuration type
export type EndpointConfig<P, R> = {
  type: "POST" | "GET" | "PUT" | "DELETE";
  path: string;
  params?: P;
  response?: R;
};

// Generic hook generators
function createMutation<P, R>(endpoint: EndpointConfig<P, R>) {
  return () => {
    return useMutation<R, Error, P>({
      mutationFn: async (data: P) => {
        return await APIClient.post<P, R>(endpoint.path, data);
      },
    });
  };
}

function createSuspenseQuery<P, R>(endpoint: EndpointConfig<P, R>) {
  return (params: P, enabled: boolean = true) => {
    return useQuery({
      queryKey: [endpoint.path, params],
      queryFn: async () => {
        return await APIClient.get<R>(endpoint.path, { params });
      },
      enabled,
    });
  };
}

export const queryEndpoint = <P, R>(config: { type: "GET"; path: string }) => {
  return { useSuspenseQuery: createSuspenseQuery<P, R>(config) };
};

export const mutationEndpoint = <P, R>(config: {
  type: "POST" | "PUT" | "DELETE";
  path: string;
}) => {
  return { useMutation: createMutation<P, R>(config) };
};
