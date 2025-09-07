import { zodResolver } from "@hookform/resolvers/zod";
import { IdCard, User } from "lucide-react";
import { type ChangeEvent, useEffect } from "react";
import { useForm } from "react-hook-form";
import z from "zod";
import { APIHooks } from "@/api/hooks";
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
import { CenteredSpineer } from "@/components/ui/spinner";

interface Props {
  open: boolean;
  setIsOpen: (open: boolean) => void;
}

const FR_SSN_REGEX =
  /([12])([0-9]{2})(0[1-9]|1[0-2])(2[AB]|[0-9]{2})[0-9]{3}[0-9]{3}([0-9]{2})/;

export const AddPatientModal = ({ open, setIsOpen }: Props) => {
  const addPatientMutation = APIHooks.patient.savePatient.useMutation();

  const addPatientFormSchema = z.object({
    firstName: z.string().trim().nonempty("First name is required"),
    lastName: z.string().trim().nonempty("Last name is required"),
    ssn: z.string().length(15).regex(FR_SSN_REGEX, {
      error: "SSN number does not match the expected format",
    }),
  });

  const addPatientForm = useForm({
    resolver: zodResolver(addPatientFormSchema),
    defaultValues: {
      firstName: "",
      lastName: "",
      ssn: "",
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
      "firstName",
      findPatientBySSNQuery.data.first_name || "",
    );
    addPatientForm.setValue(
      "lastName",
      findPatientBySSNQuery.data.last_name || "",
    );
  }, [findPatientBySSNQuery.data?.ssn]);

  const onSubmit = addPatientForm.handleSubmit(async (values) => {
    addPatientMutation
      .mutateAsync({
        first_name: values.firstName,
        last_name: values.lastName,
        ssn: values.ssn,
      })
      .then(() => setIsOpen(false))
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
            <Label htmlFor="email" className="text-sm font-medium">
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
          {findPatientBySSNQuery.data && canSearchPatient && (
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="firstName" className="text-sm font-medium">
                  First Name
                </Label>
                <FormInput
                  id="firstName"
                  name="firstName"
                  type="text"
                  placeholder="First name"
                  className="pl-10 h-11"
                  icon={
                    <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                  }
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="lastName" className="text-sm font-medium">
                  Last Name
                </Label>
                <FormInput
                  id="lastName"
                  name="lastName"
                  type="text"
                  placeholder="Last name"
                  className="pl-10 h-11"
                  icon={
                    <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                  }
                />
              </div>
            </div>
          )}
          <Button type="submit" className="w-full">
            Add patient
          </Button>
        </FormProvider>
      </DialogContent>
    </Dialog>
  );
};
