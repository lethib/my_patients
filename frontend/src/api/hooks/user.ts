import { mutationEndpoint, queryEndpoint } from "../endpointGenerator";

type SaveBusinessInformation = {
  rpps_number: string;
  siret_number: string;
  adeli_number?: string;
};

type MyOffice = {
  id: number;
  name: string;
};

export const userSchema = {
  saveBusinessInformation: mutationEndpoint<
    SaveBusinessInformation,
    { success: boolean }
  >({
    type: "POST",
    path: "/user/_save_business_information",
  }),
  getMyOffices: queryEndpoint<null, MyOffice[]>({
    type: "GET",
    path: "/user/my_offices",
  }),
};
