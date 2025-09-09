import {
  mutationEndpoint,
  type Paginated,
  queryEndpoint,
} from "../endpointGenerator";

type SavePatientParams = {
  first_name: string;
  last_name: string;
  ssn: string;
};

type SearchBySSNPatientResponse = {
  first_name: string | null;
  last_name: string | null;
  ssn: string | null;
};

interface SearchPatientParams {
  q: string;
  page: number;
}

export type SearchPatientResponse = {
  first_name: string;
  last_name: string;
  ssn: string;
};

export const patientSchema = {
  savePatient: mutationEndpoint<SavePatientParams, { success: boolean }>({
    type: "POST",
    path: "/patient/save",
  }),
  searchBySSN: queryEndpoint<{ ssn: string }, SearchBySSNPatientResponse>({
    type: "GET",
    path: "/patient/_search_by_ssn",
  }),
  search: queryEndpoint<SearchPatientParams, Paginated<SearchPatientResponse>>({
    type: "GET",
    path: "/patient/_search",
  }),
};
