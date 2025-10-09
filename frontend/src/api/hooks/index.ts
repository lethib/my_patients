import { authSchema } from "./auth";
import { patientSchema } from "./patient";
import { practitionerOfficeSchema } from "./practitioner_office";
import { userSchema } from "./user";

export const APIHooks = {
  auth: authSchema,
  patient: patientSchema,
  user: userSchema,
  office: practitionerOfficeSchema,
};
