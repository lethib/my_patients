import { zodResolver } from "@hookform/resolvers/zod";
import { IdCard, User } from "lucide-react";
import { type ChangeEvent, useEffect } from "react";
import { useForm } from "react-hook-form";
import z from "zod";
import { queryClient } from "@/api/api";
import { APIHooks } from "@/api/hooks";
import { POSSIBLE_OFFICES } from "@/api/hooks/patient";
import { FormInput } from "@/components/form/FormInput";
import { FormProvider } from "@/components/form/FormProvider";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  Label,
} from "@/components/ui";
import { FormControl, FormField, FormItem } from "@/components/ui/form";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { CenteredSpineer } from "@/components/ui/spinner";

interface Props {
  open: boolean;
  setIsOpen: (open: boolean) => void;
}

const FR_SSN_REGEX =
  /([12])([0-9]{2})(0[1-9]|1[0-2])(2[AB]|[0-9]{2})[0-9]{3}[0-9]{3}([0-9]{2})/;
const FR_ZIP_CODE_REGEX = /^(?:0[1-9]|[1-8]\d|9[0-8])\d{3}$/;

export const AddPatientModal = ({ open, setIsOpen }: Props) => {
  const addPatientMutation = APIHooks.patient.savePatient.useMutation();

  const addPatientFormSchema = z.object({
    first_name: z.string().trim().nonempty("First name is required"),
    last_name: z.string().trim().nonempty("Last name is required"),
    ssn: z.string().length(15).regex(FR_SSN_REGEX, {
      error: "SSN number does not match the expected format",
    }),
    address_line_1: z.string().trim().nonempty("Address is required"),
    address_zip_code: z.string().trim().length(5).regex(FR_ZIP_CODE_REGEX, {
      error: "Zip code does not match the expected format",
    }),
    address_city: z.string().trim().nonempty("City is required"),
    office: z.enum(POSSIBLE_OFFICES, {
      error: "Select a valid office location",
    }),
  });

  const addPatientForm = useForm({
    resolver: zodResolver(addPatientFormSchema),
    defaultValues: {
      first_name: "",
      last_name: "",
      ssn: "",
      address_line_1: "",
      address_zip_code: "",
      address_city: "",
    },
  });

  const canSearchPatient = addPatientForm.getValues("ssn").length === 15;

  const findPatientBySSNQuery = APIHooks.patient.searchBySSN.useQuery(
    { ssn: addPatientForm.getValues("ssn") },
    { enabled: canSearchPatient },
  );

  // biome-ignore lint/correctness/useExhaustiveDependencies: not needed
  useEffect(() => {
    if (!findPatientBySSNQuery.data) return;
    addPatientForm.setValue(
      "first_name",
      findPatientBySSNQuery.data.first_name,
    );
    addPatientForm.setValue("last_name", findPatientBySSNQuery.data.last_name);
    addPatientForm.setValue(
      "address_line_1",
      findPatientBySSNQuery.data.address_line_1,
    );
    addPatientForm.setValue(
      "address_zip_code",
      findPatientBySSNQuery.data.address_zip_code,
    );
    addPatientForm.setValue(
      "address_city",
      findPatientBySSNQuery.data.address_city,
    );
  }, [findPatientBySSNQuery.data?.ssn]);

  const onSubmit = addPatientForm.handleSubmit(async (values) => {
    addPatientMutation
      .mutateAsync(values)
      .then(() => {
        setIsOpen(false);
        queryClient.invalidateQueries({ queryKey: ["/patient/_search"] });
      })
      .catch((error) => alert(error.message));
  });

  const formatSSN = (value: string) => {
    const digits = value.replace(/\D/g, "");
    if (digits.length <= 1) return digits;
    if (digits.length <= 3) return `${digits[0]} ${digits.slice(1)}`;
    if (digits.length <= 5)
      return `${digits[0]} ${digits.slice(1, 3)} ${digits.slice(3)}`;
    if (digits.length <= 7)
      return `${digits[0]} ${digits.slice(1, 3)} ${digits.slice(3, 5)} ${digits.slice(5)}`;
    if (digits.length <= 10)
      return `${digits[0]} ${digits.slice(1, 3)} ${digits.slice(3, 5)} ${digits.slice(5, 7)} ${digits.slice(7)}`;
    if (digits.length <= 13)
      return `${digits[0]} ${digits.slice(1, 3)} ${digits.slice(3, 5)} ${digits.slice(5, 7)} ${digits.slice(7, 10)} ${digits.slice(10)}`;
    return `${digits[0]} ${digits.slice(1, 3)} ${digits.slice(3, 5)} ${digits.slice(5, 7)} ${digits.slice(7, 10)} ${digits.slice(10, 13)} ${digits.slice(13, 15)}`;
  };

  const handleSSNChange = (e: ChangeEvent<HTMLInputElement>) => {
    const rawValue = e.target.value.replace(/\D/g, "");
    if (rawValue.length <= 15) {
      addPatientForm.setValue("ssn", rawValue);
    }
  };

  return (
    <Dialog open={open} onOpenChange={setIsOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add a patient</DialogTitle>
          <DialogDescription>Fill the SSN to get started</DialogDescription>
        </DialogHeader>

        <FormProvider
          methods={addPatientForm}
          onSubmit={onSubmit}
          className="space-y-4"
        >
          <div className="space-y-2">
            <Label htmlFor="ssn" className="text-sm font-medium">
              Social Security Number
            </Label>
            <FormInput
              id="ssn"
              name="ssn"
              type="text"
              onChange={handleSSNChange}
              value={formatSSN(addPatientForm.watch("ssn") || "")}
              placeholder="15-digits long"
              className="pl-10 h-11"
              icon={
                <IdCard className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              }
            />
          </div>
          {findPatientBySSNQuery.isFetching && (
            <CenteredSpineer className="text-secondary" />
          )}
          {findPatientBySSNQuery.isFetched && canSearchPatient && (
            <>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="first_name" className="text-sm font-medium">
                    First Name
                  </Label>
                  <FormInput
                    id="first_name"
                    name="first_name"
                    type="text"
                    placeholder="First name"
                    className="pl-10 h-11"
                    icon={
                      <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                    }
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="last_name" className="text-sm font-medium">
                    Last Name
                  </Label>
                  <FormInput
                    id="last_name"
                    name="last_name"
                    type="text"
                    placeholder="Last name"
                    className="pl-10 h-11"
                    icon={
                      <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                    }
                  />
                </div>
              </div>
              <div className="space-y-2">
                <Label htmlFor="address_line_1" className="text-sm font-medium">
                  Address
                </Label>
                <FormInput
                  id="address_line_1"
                  name="address_line_1"
                  type="text"
                  placeholder="Street address"
                  className="h-11"
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label
                    htmlFor="address_zip_code"
                    className="text-sm font-medium"
                  >
                    Zip Code
                  </Label>
                  <FormInput
                    id="address_zip_code"
                    name="address_zip_code"
                    type="text"
                    placeholder="Zip code"
                    className="h-11"
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="address_city" className="text-sm font-medium">
                    City
                  </Label>
                  <FormInput
                    id="address_city"
                    name="address_city"
                    type="text"
                    placeholder="City"
                    className="h-11"
                  />
                </div>
              </div>

              <div className="space-y-2">
                <Label htmlFor="office" className="text-sm font-medium">
                  Office
                </Label>
                <FormField
                  name="office"
                  control={addPatientForm.control}
                  render={({ field }) => (
                    <FormItem>
                      <Select
                        onValueChange={field.onChange}
                        defaultValue={field.value}
                      >
                        <FormControl>
                          <SelectTrigger className="w-full">
                            <SelectValue placeholder="Select an office" />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          <SelectGroup>
                            {POSSIBLE_OFFICES.map((office) => (
                              <SelectItem value={office} key={office}>
                                {office}
                              </SelectItem>
                            ))}
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </FormItem>
                  )}
                />
              </div>
            </>
          )}
          <Button type="submit" className="w-full">
            Add patient
          </Button>
        </FormProvider>
      </DialogContent>
    </Dialog>
  );
};
