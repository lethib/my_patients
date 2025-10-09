import { zodResolver } from "@hookform/resolvers/zod";
import type { MutationFunction } from "@tanstack/react-query";
import { Building2, MapPin } from "lucide-react";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import z from "zod";
import { queryClient } from "@/api/api";
import type {
  PractitionerOffice,
  PractitionerOfficeParams,
} from "@/api/hooks/practitioner_office";
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

interface OfficeModalProps {
  open: boolean;
  asyncMutation: MutationFunction<
    { success: boolean },
    PractitionerOfficeParams
  >;
  setIsOpen: (open: boolean) => void;
  office?: PractitionerOffice | null;
}

const FR_ZIP_CODE_REGEX = /^(?:0[1-9]|[1-8]\d|9[0-8])\d{3}$/;

export const OfficeModal = ({
  open,
  asyncMutation,
  setIsOpen,
  office,
}: OfficeModalProps) => {
  const { t } = useTranslation();

  const isEditMode = !!office;

  const officeFormSchema = z.object({
    name: z.string().trim().min(1, t("offices.form.validation.nameRequired")),
    address_line_1: z
      .string()
      .trim()
      .min(1, t("offices.form.validation.addressRequired")),
    address_zip_code: z
      .string()
      .trim()
      .length(5)
      .regex(FR_ZIP_CODE_REGEX, {
        message: t("offices.form.validation.zipCodeInvalid"),
      }),
    address_city: z
      .string()
      .trim()
      .min(1, t("offices.form.validation.cityRequired")),
  });

  const officeForm = useForm({
    resolver: zodResolver(officeFormSchema),
    defaultValues: {
      name: "",
      address_line_1: "",
      address_zip_code: "",
      address_city: "",
    },
  });

  useEffect(() => {
    if (office) {
      officeForm.reset({
        name: office.name,
        address_line_1: office.address_line_1,
        address_zip_code: office.address_zip_code,
        address_city: office.address_city,
      });
    } else {
      officeForm.reset({
        name: "",
        address_line_1: "",
        address_zip_code: "",
        address_city: "",
      });
    }
  }, [office, officeForm]);

  const onSubmit = officeForm.handleSubmit(async (values) => {
    asyncMutation(values)
      .then(() => {
        queryClient.invalidateQueries({ queryKey: ["/user/my_offices"] });
        setIsOpen(false);
        officeForm.reset();
      })
      .catch((error) => alert((error as Error).message));
  });

  return (
    <Dialog open={open} onOpenChange={setIsOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {isEditMode
              ? t("offices.form.editTitle")
              : t("offices.form.addTitle")}
          </DialogTitle>
          <DialogDescription>
            {isEditMode
              ? t("offices.form.editDescription")
              : t("offices.form.addDescription")}
          </DialogDescription>
        </DialogHeader>

        <FormProvider
          methods={officeForm}
          onSubmit={onSubmit}
          className="space-y-4"
        >
          <div className="space-y-2">
            <Label htmlFor="name" className="text-sm font-medium">
              {t("offices.form.name")}
            </Label>
            <FormInput
              id="name"
              name="name"
              type="text"
              placeholder={t("offices.form.namePlaceholder")}
              className="pl-10 h-11"
              icon={
                <Building2 className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              }
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="address_line_1" className="text-sm font-medium">
              {t("offices.form.address")}
            </Label>
            <FormInput
              id="address_line_1"
              name="address_line_1"
              type="text"
              placeholder={t("offices.form.addressPlaceholder")}
              className="pl-10 h-11"
              icon={
                <MapPin className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              }
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="address_zip_code" className="text-sm font-medium">
                {t("offices.form.zipCode")}
              </Label>
              <FormInput
                id="address_zip_code"
                name="address_zip_code"
                type="text"
                className="h-11"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="address_city" className="text-sm font-medium">
                {t("offices.form.city")}
              </Label>
              <FormInput
                id="address_city"
                name="address_city"
                type="text"
                className="h-11"
              />
            </div>
          </div>

          <Button type="submit" className="w-full">
            {isEditMode ? t("offices.form.update") : t("offices.form.add")}
          </Button>
        </FormProvider>
      </DialogContent>
    </Dialog>
  );
};
