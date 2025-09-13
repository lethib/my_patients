import { zodResolver } from "@hookform/resolvers/zod";
import { createFileRoute } from "@tanstack/react-router";
import { Building2, FileText, Users } from "lucide-react";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import z from "zod";
import { APIHooks } from "@/api/hooks";
import { FormInput } from "@/components/form/FormInput";
import { FormProvider } from "@/components/form/FormProvider";
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Label,
} from "@/components/ui";
import { CenteredSpineer } from "@/components/ui/spinner";
import { useCurrentUser } from "@/hooks/useCurrentUser";

export const Route = createFileRoute("/my_information")({
  component: MyInformation,
});

const businessInfoSchema = z.object({
  rpps_number: z.string().trim().length(11),
  siret_number: z.string().trim().length(14),
  adeli_number: z.string().trim().optional(),
});

function MyInformation() {
  const { currentUser } = useCurrentUser();

  const saveBusinessInformationMutation =
    APIHooks.user.saveBusinessInformation.useMutation();

  const businessForm = useForm({
    resolver: zodResolver(businessInfoSchema),
    defaultValues: {
      rpps_number: "",
      siret_number: "",
      adeli_number: "",
    },
  });

  useEffect(() => {
    if (currentUser?.business_information) {
      businessForm.reset({
        rpps_number: currentUser.business_information.rpps_number || "",
        siret_number: currentUser.business_information.siret_number || "",
        adeli_number: currentUser.business_information.adeli_number || "",
      });
    }
  }, [currentUser, businessForm]);

  const onSubmit = businessForm.handleSubmit(async (values) => {
    saveBusinessInformationMutation
      .mutateAsync(values)
      .then(() => alert("Business information saved successfully!"));
  });

  return (
    <div className="container mx-auto p-6 max-w-2xl">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Building2 className="h-5 w-5" />
            Business Information
          </CardTitle>
          <CardDescription>
            Enter your professional identification numbers
          </CardDescription>
        </CardHeader>
        <CardContent>
          <FormProvider
            methods={businessForm}
            onSubmit={onSubmit}
            className="space-y-6"
          >
            <div className="space-y-2">
              <Label htmlFor="rpps_number" className="text-sm font-medium">
                RPPS Number *
              </Label>
              <FormInput
                id="rpps_number"
                name="rpps_number"
                type="text"
                placeholder="Enter your RPPS number"
                className="pl-10 h-11"
                icon={
                  <Users className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="siret_number" className="text-sm font-medium">
                SIRET Number *
              </Label>
              <FormInput
                id="siret_number"
                name="siret_number"
                type="text"
                placeholder="Enter your SIRET number"
                className="pl-10 h-11"
                icon={
                  <Building2 className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="adeli_number" className="text-sm font-medium">
                ADELI Number
              </Label>
              <FormInput
                id="adeli_number"
                name="adeli_number"
                type="text"
                placeholder="Enter your ADELI number (optional)"
                className="pl-10 h-11"
                icon={
                  <FileText className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>

            <Button type="submit" className="w-full">
              Save Business Information
            </Button>
          </FormProvider>
        </CardContent>
      </Card>
    </div>
  );
}
