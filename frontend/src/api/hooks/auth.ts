import { mutationEndpoint, queryEndpoint } from "../endpointGenerator";

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
  first_name: string;
  last_name: string;
  email: string;
  password: string;
  phone_number: string;
};

export type MeResponse = {
  pid: string;
  email: string;
  name: string;
  business_information: {
    rpps_number: string;
    siret_number: string;
    adeli_number: string | null;
    signature_filename: string | null;
  } | null;
};

type CheckAccessKeyParams = {
  access_key: string;
  user_email: string;
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
  me: queryEndpoint<null, MeResponse>({
    type: "GET",
    path: "/auth/me",
  }),
  checkAccessKey: mutationEndpoint<CheckAccessKeyParams, { token: string }>({
    type: "POST",
    path: "/auth/_check_access_key",
  }),
};
