import { mutationEndpoint, queryEndpoint } from "../endpointGenerator";

type SavePatientParams = {
  first_name: string;
  last_name: string;
  ssn: string;
};

type PatientResponse = {
  first_name: string | null;
  last_name: string | null;
  ssn: string | null;
};

export const patientSchema = {
  savePatient: mutationEndpoint<SavePatientParams, { success: boolean }>({
    type: "POST",
    path: "/patient/save",
  }),
  searchBySSN: queryEndpoint<{ ssn: string }, PatientResponse>({
    type: "GET",
    path: "/patient/_search_by_ssn",
  }),
};
