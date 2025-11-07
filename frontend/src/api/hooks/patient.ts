import { useMutation } from "@tanstack/react-query";
import type { UUID } from "crypto";
import { APIClient } from "../api";
import {
  mutationEndpoint,
  type Paginated,
  queryEndpoint,
} from "../endpointGenerator";

export type SavePatientParams = {
  pid?: string;
  first_name: string;
  last_name: string;
  email: string;
  ssn: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
  practitioner_office_id: number;
};

export type SearchBySSNPatientResponse = {
  id: number;
  pid: UUID;
  first_name: string;
  last_name: string;
  email: string;
  ssn: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
  address_country: string;
};

interface SearchPatientParams {
  q: string;
  page: number;
}

export type SearchPatientResponse = {
  id: number;
  pid: UUID;
  first_name: string;
  last_name: string;
  email: string | null;
  ssn: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
  address_country: string;
  office: { id: number; name: string } | null;
};

export const patientSchema = {
  createPatient: mutationEndpoint<SavePatientParams, { success: boolean }>({
    type: "POST",
    path: "/patient/create",
  }),
  updatePatient: mutationEndpoint<SavePatientParams, { success: boolean }>({
    type: "PUT",
    path: "/patient/{patient_id}",
  }),
  searchBySSN: queryEndpoint<{ ssn: string }, SearchBySSNPatientResponse[]>({
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
          invoice_date,
          should_be_sent_by_email,
        }: {
          patientId: number;
          amount: string;
          invoice_date: string;
          should_be_sent_by_email: boolean;
        }) => {
          const response = await APIClient.client.post<{
            pdf_data: string;
            filename: string;
          }>(`/patient/${patientId}/_generate_invoice`, {
            amount,
            invoice_date,
            should_be_sent_by_email,
          });

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
