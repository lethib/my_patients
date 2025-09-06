import { authSchema } from "./auth";
import { patientSchema } from "./patient";

export const APIHooks = {
  auth: authSchema,
  patient: patientSchema,
};
