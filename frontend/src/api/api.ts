import { QueryClient } from "@tanstack/react-query";
import axios, {
  AxiosError,
  type AxiosInstance,
  type AxiosRequestConfig,
} from "axios";
import { APIHooks } from "./hooks";

class MyPatientsAPI {
  client: AxiosInstance;
  hooks: typeof APIHooks;

  constructor(baseURL: string) {
    this.client = axios.create({ baseURL });
    this.hooks = APIHooks;

    this.client.interceptors.request.use(
      (config) => {
        const accessToken = localStorage.getItem("accessToken");
        if (accessToken) {
          config.headers.Authorization = `Bearer ${accessToken}`;
        }
        return config;
      },
      (error: AxiosError) => Promise.reject(error),
    );
  }

  get = async <R>(path: string, params?: AxiosRequestConfig): Promise<R> => {
    const res = await this.client.get<R>(path, { params });
    return res.data;
  };

  post = async <P, R>(
    path: string,
    data: P,
    config?: AxiosRequestConfig,
  ): Promise<R> => {
    const res = await this.client.post<R>(path, data, config);
    return res.data;
  };
}

export const APIClient = new MyPatientsAPI(import.meta.env.VITE_BASE_API_URL);

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 2,
      refetchOnWindowFocus: false,
    },
  },
});
