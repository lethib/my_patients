import { mutationEndpoint } from "../endpointGenerator";

type SavePatientParams = {
  first_name: string;
  last_name: string;
  ssn: string;
};

export const patientSchema = {
  savePatient: mutationEndpoint<SavePatientParams, { success: boolean }>({
    type: "POST",
    path: "/patient/save",
  }),
};
