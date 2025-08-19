import { mutationEndpoint } from "../endpointGenetator";

type LoginParams = {
  email: string;
  password: string;
};

type AuthResponse = {
  token: string;
  pid: string;
  name: string;
  is_verified: boolean;
};

type RegisterParams = {
  email: string;
  password: string;
  name: string;
};

export const authSchema = {
  login: mutationEndpoint<LoginParams, AuthResponse>({
    type: "POST",
    path: "/auth/login",
  }),
  register: mutationEndpoint<RegisterParams, null>({
    type: "POST",
    path: "/auth/register",
  }),
};
