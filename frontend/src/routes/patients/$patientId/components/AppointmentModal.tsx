import { zodResolver } from "@hookform/resolvers/zod";
import { Calendar, Euro, MapPin } from "lucide-react";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import z from "zod";
import { APIHooks } from "@/api/hooks";
import type { MedicalAppointment } from "@/api/hooks/appointment";
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
  selectedAppointment: MedicalAppointment | null;
}

export const AppointmentModal = ({
  open,
  onOpenChange,
  patientId,
  selectedAppointment,
}: Props) => {
  const { t } = useTranslation();
  const isEditMode = !!selectedAppointment;

  const officesQuery = APIHooks.user.getMyOffices.useQuery(null);
  const createMutation = APIHooks.appointment.create.useMutation();
  const updateMutation = APIHooks.appointment.update.useMutation();

  const appointmentSchema = z.object({
    date: z.date({
      message: t(
        "appointments.form.validation.dateRequired",
        "La date est requise"
      ),
    }),
    practitioner_office_id: z
      .string()
      .min(
        1,
        t(
          "appointments.form.validation.officeRequired",
          "Le cabinet est requis"
        )
      ),
    price: z
      .number({
        message: t(
          "appointments.form.validation.priceRequired",
          "Le prix est requis"
        ),
      })
      .min(
        0.01,
        t(
          "appointments.form.validation.priceRequired",
          "Le prix doit être positif"
        )
      )
      .refine((val) => Math.round(val * 100) === val * 100, {
        message: t(
          "appointments.form.validation.priceInvalid",
          "Le prix ne peut avoir que 2 décimales maximum"
        ),
      }),
  });

  type AppointmentFormData = z.infer<typeof appointmentSchema>;

  const form = useForm<AppointmentFormData>({
    resolver: zodResolver(appointmentSchema),
    defaultValues: {
      date: new Date(),
      practitioner_office_id: "",
      price: 0,
    },
  });

  // Auto-select office if only one exists
  useEffect(() => {
    if (officesQuery.data?.length === 1 && !selectedAppointment) {
      form.setValue(
        "practitioner_office_id",
        officesQuery.data[0].id.toString()
      );
    }
  }, [officesQuery.data, form, selectedAppointment]);

  // Populate form when editing
  useEffect(() => {
    if (open && selectedAppointment) {
      form.reset({
        date: new Date(selectedAppointment.date),
        practitioner_office_id:
          selectedAppointment.practitioner_office_id.toString(),
        price: selectedAppointment.price_in_cents / 100,
      });
    } else if (open && !selectedAppointment) {
      form.reset({
        date: new Date(),
        practitioner_office_id:
          officesQuery.data?.length === 1
            ? officesQuery.data[0].id.toString()
            : "",
        price: 0,
      });
    }
  }, [open, selectedAppointment, form, officesQuery.data]);

  const handleClose = () => {
    form.reset();
    onOpenChange(false);
  };

  const onSubmit = form.handleSubmit(async (data) => {
    // Format date as YYYY-MM-DD
    const year = data.date.getFullYear();
    const month = String(data.date.getMonth() + 1).padStart(2, "0");
    const day = String(data.date.getDate()).padStart(2, "0");
    const dateString = `${year}-${month}-${day}`;

    const params = {
      patient_id: patientId,
      practitioner_office_id: Number(data.practitioner_office_id),
      date: dateString,
      price_in_cents: Math.round(data.price * 100),
    };

    try {
      if (isEditMode) {
        await updateMutation.mutateAsync({
          id: selectedAppointment.id,
          ...params,
        });
      } else {
        await createMutation.mutateAsync(params);
      }
      handleClose();
    } catch (error) {
      console.error("Failed to save appointment:", error);
    }
  });

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Calendar className="h-5 w-5" />
            {t(
              `appointments.form.title.${isEditMode ? "edit" : "create"}`,
              isEditMode ? "Modifier le rendez-vous" : "Nouveau rendez-vous"
            )}
          </DialogTitle>
          <DialogDescription>
            {t(
              "appointments.form.description",
              "Enregistrer les détails du rendez-vous médical"
            )}
          </DialogDescription>
        </DialogHeader>

        <FormProvider methods={form} onSubmit={onSubmit} className="space-y-4">
          <FormDatePicker
            name="date"
            label={t("appointments.form.date", "Date du rendez-vous")}
            disabled={createMutation.isPending || updateMutation.isPending}
          />

          <div className="space-y-2">
            <Label htmlFor="office">
              <div className="flex items-center gap-2">
                <MapPin className="h-4 w-4" />
                {t("appointments.form.office", "Cabinet")}
              </div>
            </Label>
            <FormSelect
              name="practitioner_office_id"
              placeholder={t(
                "appointments.form.selectOffice",
                "Sélectionner un cabinet"
              )}
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
                {t("appointments.form.price", "Prix")} (€)
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
                disabled={createMutation.isPending || updateMutation.isPending}
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
              disabled={createMutation.isPending || updateMutation.isPending}
            >
              {t("common.cancel", "Annuler")}
            </Button>
            <Button
              type="submit"
              disabled={createMutation.isPending || updateMutation.isPending}
            >
              {t(
                `appointments.form.submit.${isEditMode ? "edit" : "create"}`,
                isEditMode
                  ? "Modifier le rendez-vous"
                  : "Créer le rendez-vous"
              )}
            </Button>
          </div>
        </FormProvider>
      </DialogContent>
    </Dialog>
  );
};
