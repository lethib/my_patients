import { useMutation, useQuery } from "@tanstack/react-query";
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
  deletePatient: mutationEndpoint<null, null>({
    type: "DELETE",
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
          practitioner_office_id,
        }: {
          patientId: number;
          amount: number;
          invoice_date: string;
          should_be_sent_by_email: boolean;
          practitioner_office_id: number;
        }) => {
          const response = await APIClient.client.post<{
            pdf_data: string;
            filename: string;
          }>(`/patient/${patientId}/_generate_invoice`, {
            amount,
            invoice_date,
            should_be_sent_by_email,
            practitioner_office_id,
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
  getById: {
    useQuery: (patientId: number) => {
      return useQuery<SearchPatientResponse>({
        queryKey: ["/patient/by_id", patientId],
        queryFn: async () => {
          // Simulate API delay
          await new Promise((resolve) => setTimeout(resolve, 300));

          // Generate mock patient data based on ID
          const firstNames = [
            "Jean",
            "Marie",
            "Pierre",
            "Sophie",
            "Luc",
            "Claire",
            "Marc",
            "Julie",
          ];
          const lastNames = [
            "Dupont",
            "Martin",
            "Bernard",
            "Thomas",
            "Robert",
            "Petit",
            "Durand",
            "Leroy",
          ];
          const cities = [
            "Paris",
            "Lyon",
            "Marseille",
            "Toulouse",
            "Bordeaux",
            "Lille",
            "Nice",
            "Nantes",
          ];
          const streets = [
            "Rue de la République",
            "Avenue des Champs",
            "Boulevard Victor Hugo",
            "Place de la Liberté",
            "Rue du Commerce",
            "Avenue Jean Jaurès",
            "Rue Nationale",
            "Boulevard Gambetta",
          ];

          const firstNameIndex = patientId % firstNames.length;
          const lastNameIndex = Math.floor(patientId / 10) % lastNames.length;
          const cityIndex = patientId % cities.length;
          const streetIndex =
            Math.floor(patientId / 3) % streets.length;

          // Generate realistic SSN
          const gender = patientId % 2 === 0 ? "2" : "1";
          const year = String(50 + (patientId % 50)).padStart(2, "0");
          const month = String(1 + (patientId % 12)).padStart(2, "0");
          const department = String(1 + (patientId % 95)).padStart(2, "0");
          const commune = String(1 + (patientId % 999)).padStart(3, "0");
          const order = String(1 + (patientId % 999)).padStart(3, "0");
          const key = String((97 - (patientId % 97))).padStart(2, "0");
          const ssn = `${gender}${year}${month}${department}${commune}${order}${key}`;

          return {
            id: patientId,
            pid: `00000000-0000-0000-0000-${String(patientId).padStart(12, "0")}` as UUID,
            first_name: firstNames[firstNameIndex],
            last_name: lastNames[lastNameIndex],
            email: `${firstNames[firstNameIndex].toLowerCase()}.${lastNames[lastNameIndex].toLowerCase()}@example.com`,
            ssn,
            address_line_1: `${10 + (patientId % 90)} ${streets[streetIndex]}`,
            address_zip_code: `${75000 + (patientId % 999)}`,
            address_city: cities[cityIndex],
            address_country: "France",
          };
        },
      });
    },
  },
};
