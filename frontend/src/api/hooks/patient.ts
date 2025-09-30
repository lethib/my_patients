import { useMutation } from "@tanstack/react-query";
import { APIClient } from "../api";
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
  practitioner_office_id: number;
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
  generateInvoice: {
    useMutation: () => {
      return useMutation({
        mutationFn: async ({
          patientId,
          amount,
        }: {
          patientId: number;
          amount: string;
        }) => {
          const response = await APIClient.client.post<{
            pdf_data: string;
            filename: string;
          }>(`/patient/${patientId}/_generate_invoice`, { amount });

          // Decode base64 PDF data to blob
          const pdfData = response.data.pdf_data;
          const binaryString = atob(pdfData);
          const bytes = new Uint8Array(binaryString.length);
          for (let i = 0; i < binaryString.length; i++) {
            bytes[i] = binaryString.charCodeAt(i);
          }
          const blob = new Blob([bytes], { type: "application/pdf" });

          return { blob, filename: response.data.filename };
        },
      });
    },
  },
};
