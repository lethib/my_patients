import {
  mutationEndpoint,
  type Paginated,
  queryEndpoint,
} from "../endpointGenerator";

export const POSSIBLE_OFFICES = ["RueilMalmaison", "VitrySurSeine"] as const;
type Office = (typeof POSSIBLE_OFFICES)[number];

type SavePatientParams = {
  first_name: string;
  last_name: string;
  ssn: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
  office: Office;
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
  office: string;
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
