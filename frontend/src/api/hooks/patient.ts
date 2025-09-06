import { mutationEndpoint } from "../endpointGenerator";

type SavePatientParams = {
  name: string;
  ssn: string;
};

export const patientSchema = {
  savePatient: mutationEndpoint<SavePatientParams, { success: boolean }>({
    type: "POST",
    path: "/patient/save",
  }),
};
