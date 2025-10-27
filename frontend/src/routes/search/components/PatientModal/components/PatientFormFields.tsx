import { User } from "lucide-react";
import { useFormContext } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { APIHooks } from "@/api/hooks";
import { FormInput } from "@/components/form/FormInput";
import { Label } from "@/components/ui";
import { FormControl, FormField, FormItem } from "@/components/ui/form";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export const PatientFormFields = () => {
  const { t } = useTranslation();
  const { control } = useFormContext();

  const myOfficesQuery = APIHooks.user.getMyOffices.useQuery(null);

  return (
    <>
      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="first_name" className="text-sm font-medium">
            {t("patients.form.firstName")}
          </Label>
          <FormInput
            id="first_name"
            name="first_name"
            type="text"
            placeholder={t("patients.form.firstNamePlaceholder")}
            className="pl-10 h-11"
            icon={
              <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            }
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="last_name" className="text-sm font-medium">
            {t("patients.form.lastName")}
          </Label>
          <FormInput
            id="last_name"
            name="last_name"
            type="text"
            placeholder={t("patients.form.lastNamePlaceholder")}
            className="pl-10 h-11"
            icon={
              <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            }
          />
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="email" className="text-sm font-medium">
          {t("patients.form.email")}
        </Label>
        <FormInput
          id="email"
          name="email"
          type="email"
          placeholder={t("patients.form.emailPlaceholder")}
          className="h-11"
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="address_line_1" className="text-sm font-medium">
          {t("patients.form.address")}
        </Label>
        <FormInput
          id="address_line_1"
          name="address_line_1"
          type="text"
          placeholder={t("patients.form.addressPlaceholder")}
          className="h-11"
        />
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="address_zip_code" className="text-sm font-medium">
            {t("patients.form.zipCode")}
          </Label>
          <FormInput
            id="address_zip_code"
            name="address_zip_code"
            type="text"
            placeholder={t("patients.form.zipCodePlaceholder")}
            className="h-11"
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="address_city" className="text-sm font-medium">
            {t("patients.form.city")}
          </Label>
          <FormInput
            id="address_city"
            name="address_city"
            type="text"
            placeholder={t("patients.form.cityPlaceholder")}
            className="h-11"
          />
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="office" className="text-sm font-medium">
          {t("patients.form.office")}
        </Label>
        <FormField
          name="practitioner_office_id"
          control={control}
          render={({ field }) => (
            <FormItem>
              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger className="w-full">
                    <SelectValue
                      placeholder={t("patients.form.officePlaceholder")}
                    />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectGroup>
                    {myOfficesQuery.data?.map((office) => (
                      <SelectItem value={office.id.toString()} key={office.id}>
                        {office.name}
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
  );
};
