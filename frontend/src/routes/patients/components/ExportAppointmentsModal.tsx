import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import z from "zod";
import { APIHooks } from "@/api/hooks";
import { FormDatePicker } from "@/components/form/FormDatePicker";
import { FormProvider } from "@/components/form/FormProvider";
import { Button } from "@/components/ui";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const formatDate = (date: Date): string => {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
};

export const ExportAppointmentsModal = ({ open, onOpenChange }: Props) => {
  const { t } = useTranslation();
  const [showSuccessDialog, setShowSuccessDialog] = useState(false);
  const exportAppointmentsMutation =
    APIHooks.user.extractMedicalAppointment.useMutation();

  const exportAppointmentsSchema = z.object({
    start_date: z.date({
      message: t("appointments.export.validation.startDateRequired"),
    }),
    end_date: z.date({
      message: t("appointments.export.validation.endDateRequired"),
    }),
  });

  const exportAppointmentForm = useForm({
    resolver: zodResolver(exportAppointmentsSchema),
    defaultValues: {
      end_date: new Date(),
    },
  });

  const onSubmit = exportAppointmentForm.handleSubmit((values) => {
    exportAppointmentsMutation
      .mutateAsync({
        start_date: formatDate(values.start_date),
        end_date: formatDate(values.end_date),
      })
      .then(() => {
        onOpenChange(false);
        setShowSuccessDialog(true);
        exportAppointmentForm.reset();
      })
      .catch((err) => {
        exportAppointmentForm.setError("start_date", {
          message: t(`common.errors.${err.response.data.msg}`),
        });
      });
  });

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("appointments.export.title")}</DialogTitle>
            <DialogDescription>
              {t("appointments.export.description")}
            </DialogDescription>
          </DialogHeader>

          <FormProvider
            methods={exportAppointmentForm}
            onSubmit={onSubmit}
            className="space-y-4"
          >
            <div className="flex gap-4">
              <FormDatePicker
                name="start_date"
                label={t("appointments.export.startDate")}
              />
              <FormDatePicker
                name="end_date"
                label={t("appointments.export.endDate")}
              />
            </div>

            <Button
              type="submit"
              disabled={exportAppointmentForm.formState.isSubmitting}
              className="w-full"
            >
              {t("appointments.export.submit")}
            </Button>
          </FormProvider>
        </DialogContent>
      </Dialog>

      <Dialog open={showSuccessDialog} onOpenChange={setShowSuccessDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("appointments.export.success.title")}</DialogTitle>
            <DialogDescription>
              {t("appointments.export.success.description")}
            </DialogDescription>
          </DialogHeader>

          <Button
            onClick={() => setShowSuccessDialog(false)}
            className="w-full"
          >
            {t("common.close")}
          </Button>
        </DialogContent>
      </Dialog>
    </>
  );
};
