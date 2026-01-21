import { zodResolver } from "@hookform/resolvers/zod";
import { Calendar, Euro, MapPin } from "lucide-react";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import z from "zod";
import { queryClient } from "@/api/api";
import { APIHooks } from "@/api/hooks";
import { FormDatePicker } from "@/components/form/FormDatePicker";
import { FormInput } from "@/components/form/FormInput";
import { FormProvider } from "@/components/form/FormProvider";
import { FormSelect } from "@/components/form/FormSelect";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui";
import { Label } from "@/components/ui/label";

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  patientId: number;
}

export const AppointmentModal = ({ open, onOpenChange, patientId }: Props) => {
  const { t } = useTranslation();

  const officesQuery = APIHooks.user.getMyOffices.useQuery(null, {
    enabled: open,
  });
  const createAppointmentMutation = APIHooks.patient
    .createMedicalAppointment(patientId)
    .useMutation();

  const appointmentSchema = z.object({
    date: z.date({
      message: t("appointments.form.validation.dateRequired"),
    }),
    practitioner_office_id: z
      .string()
      .min(1, t("appointments.form.validation.officeRequired")),
    price: z.coerce
      .number<number>()
      .min(1, t("invoice.errors.invalidAmount"))
      .refine(
        (val) => {
          // Check if the number has at most 2 decimal places
          // by multiplying by 100 and checking if it's an integer
          return Math.round(val * 100) === val * 100;
        },
        { message: t("invoice.errors.invalidAmount") },
      ),
  });

  const form = useForm({
    resolver: zodResolver(appointmentSchema),
    defaultValues: {
      date: new Date(),
      practitioner_office_id: "",
      price: 0,
    },
  });

  // Auto-select office if only one exists
  useEffect(() => {
    if (officesQuery.data?.length === 1) {
      form.setValue(
        "practitioner_office_id",
        officesQuery.data[0].id.toString(),
      );
    }
  }, [officesQuery.data, form]);

  const handleClose = () => {
    form.reset();
    onOpenChange(false);
  };

  const onSubmit = form.handleSubmit(async (data) => {
    // // Format date as YYYY-MM-DD
    // const year = data.date.getFullYear();
    // const month = String(data.date.getMonth() + 1).padStart(2, "0");
    // const day = String(data.date.getDate()).padStart(2, "0");
    // const dateString = `${year}-${month}-${day}`;

    createAppointmentMutation
      .mutateAsync({
        practitioner_office_id: Number(data.practitioner_office_id),
        date: data.date,
        price_in_cents: Math.round(data.price * 100),
      })
      .then(() => {
        queryClient.invalidateQueries({
          queryKey: [`/patient/${patientId}/medical_appointments`, null],
        });
        handleClose();
      });
  });

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Calendar className="h-5 w-5" />
            {t("appointments.form.title.create")}
          </DialogTitle>
          <DialogDescription>
            {t("appointments.form.description")}
          </DialogDescription>
        </DialogHeader>

        <FormProvider methods={form} onSubmit={onSubmit} className="space-y-4">
          <FormDatePicker name="date" label={t("appointments.form.date")} />

          <div className="space-y-2">
            <Label htmlFor="office">
              <div className="flex items-center gap-2">
                <MapPin className="h-4 w-4" />
                {t("appointments.form.office")}
              </div>
            </Label>
            <FormSelect
              name="practitioner_office_id"
              placeholder={t("appointments.form.selectOffice")}
              options={
                officesQuery.data?.map((office) => ({
                  value: office.id.toString(),
                  label: office.name,
                })) || []
              }
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="price">
              <div className="flex items-center gap-2">
                <Euro className="h-4 w-4" />
                {t("appointments.form.price")} (€)
              </div>
            </Label>
            <div className="relative">
              <FormInput
                id="price"
                name="price"
                type="number"
                step="0.01"
                min="0"
                placeholder="60.00"
                className="pr-8"
              />
              <div className="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                <span className="text-muted-foreground text-sm">€</span>
              </div>
            </div>
          </div>

          <div className="flex gap-2 justify-end pt-4">
            <Button
              type="button"
              variant="outline"
              onClick={handleClose}
              disabled={form.formState.isSubmitting}
            >
              {t("common.cancel")}
            </Button>
            <Button type="submit" disabled={form.formState.isSubmitting}>
              {t("appointments.form.submit.create")}
            </Button>
          </div>
        </FormProvider>
      </DialogContent>
    </Dialog>
  );
};
