import { mutationEndpoint } from "../endpointGenerator";

export type PractitionerOffice = {
  id: number;
  name: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
};

export type PractitionerOfficeParams = {
  name: string;
  address_line_1: string;
  address_zip_code: string;
  address_city: string;
};

export const practitionerOfficeSchema = {
  createOffice: mutationEndpoint<
    PractitionerOfficeParams,
    { success: boolean }
  >({
    type: "POST",
    path: "/practitioner_office/create",
  }),
  updateOffice: mutationEndpoint<
    PractitionerOfficeParams,
    { success: boolean }
  >({
    type: "PUT",
    path: "/practitioner_office/{office_id}",
  }),
};
