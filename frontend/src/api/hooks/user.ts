import { mutationEndpoint } from "../endpointGenerator";

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
};
