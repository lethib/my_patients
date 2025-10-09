import { mutationEndpoint, queryEndpoint } from "../endpointGenerator";
import type { PractitionerOffice } from "./practitioner_office";

type SaveBusinessInformation = {
  rpps_number: string;
  siret_number: string;
  adeli_number?: string;
};

export const userSchema = {
  saveBusinessInformation: mutationEndpoint<
    SaveBusinessInformation,
    { success: boolean }
  >({
    type: "POST",
    path: "/user/_save_business_information",
  }),
  getMyOffices: queryEndpoint<null, PractitionerOffice[]>({
    type: "GET",
    path: "/user/my_offices",
  }),
};
