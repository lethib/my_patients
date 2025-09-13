import { authSchema } from "./auth";
import { patientSchema } from "./patient";
import { userSchema } from "./user";

export const APIHooks = {
  auth: authSchema,
  patient: patientSchema,
  user: userSchema,
};
