import {
  type UseQueryOptions,
  useMutation,
  useQuery,
} from "@tanstack/react-query";
import type { AxiosError } from "axios";
import { APIClient, type APIError } from "./api";

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
    return useMutation<R, AxiosError<APIError>, P>({
      mutationFn: async (data: P) => {
        return await APIClient.post<P, R>(endpoint.path, data);
      },
    });
  };
}

function createQuery<P, R>(endpoint: EndpointConfig<P, R>) {
  return (
    params: P,
    options?: Omit<
      UseQueryOptions<R, AxiosError<APIError>, R>,
      "queryKey" | "queryFn"
    >,
  ) => {
    return useQuery({
      queryKey: [endpoint.path],
      queryFn: async () => {
        return await APIClient.get<R>(endpoint.path, { params });
      },
      ...options,
    });
  };
}

export const queryEndpoint = <P, R>(config: { type: "GET"; path: string }) => {
  return { useQuery: createQuery<P, R>(config) };
};

export const mutationEndpoint = <P, R>(config: {
  type: "POST" | "PUT" | "DELETE";
  path: string;
}) => {
  return { useMutation: createMutation<P, R>(config) };
};
