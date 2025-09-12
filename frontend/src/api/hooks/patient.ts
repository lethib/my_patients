import {
  mutationEndpoint,
  type Paginated,
  queryEndpoint,
} from "../endpointGenerator";

type SavePatientParams = {
  first_name: string;
  last_name: string;
  ssn: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
};

type SearchBySSNPatientResponse = {
  first_name: string;
  last_name: string;
  ssn: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
  address_country: string;
} | null;

interface SearchPatientParams {
  q: string;
  page: number;
}

export type SearchPatientResponse = {
  id: number;
  first_name: string;
  last_name: string;
  ssn: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
  address_country: string;
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
