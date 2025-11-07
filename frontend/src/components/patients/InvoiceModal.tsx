import { zodResolver } from "@hookform/resolvers/zod";
import { useNavigate } from "@tanstack/react-router";
import { CircleAlert, FileText, Loader2 } from "lucide-react";
import React from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { z } from "zod";
import { patientSchema, type SearchPatientResponse } from "@/api/hooks/patient";
import { FormDatePicker } from "@/components/form/FormDatePicker";
import { FormInput } from "@/components/form/FormInput";
import { FormProvider } from "@/components/form/FormProvider";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { useCurrentUser } from "@/hooks/useCurrentUser";
import { FormSwitch } from "../form/FormSwitch";

interface InvoiceModalProps {
  isOpen: boolean;
  onClose: () => void;
  patient: SearchPatientResponse;
}

export const InvoiceModal: React.FC<InvoiceModalProps> = ({
  isOpen,
  onClose,
  patient,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { currentUser } = useCurrentUser();

  const generateInvoiceMutation = patientSchema.generateInvoice.useMutation();

  const invoiceFormSchema = z.object({
    amount: z
      .string()
      .min(1, t("invoice.errors.invalidAmount"))
      .refine(
        (val) => {
          const num = parseFloat(val);
          return !isNaN(num) && num > 0;
        },
        { message: t("invoice.errors.invalidAmount") },
      ),
    date: z.date(),
    shouldSendInvoiceByEmail: z.boolean(),
  });

  type InvoiceFormData = z.infer<typeof invoiceFormSchema>;

  const invoiceForm = useForm<InvoiceFormData>({
    resolver: zodResolver(invoiceFormSchema),
    defaultValues: {
      amount: "",
      date: new Date(),
      shouldSendInvoiceByEmail: false,
    },
  });

  const onSubmit = invoiceForm.handleSubmit(async (data) => {
    const numericAmount = parseFloat(data.amount);

    // Format date as YYYY-MM-DD using local timezone (not UTC)
    const year = data.date.getFullYear();
    const month = String(data.date.getMonth() + 1).padStart(2, "0");
    const day = String(data.date.getDate()).padStart(2, "0");

    generateInvoiceMutation.mutateAsync(
      {
        patientId: patient.id,
        amount: `${numericAmount}€`,
        invoice_date: `${year}-${month}-${day}`,
      },
      {
        onSuccess: ({ blob, filename }) => {
          const url = window.URL.createObjectURL(blob);
          const link = document.createElement("a");
          link.href = url;
          link.download = filename;
          document.body.appendChild(link);
          link.click();
          document.body.removeChild(link);
          window.URL.revokeObjectURL(url);
          invoiceForm.reset();
          onClose();
        },
      },
    );
  });

  const handleClose = () => {
    if (!generateInvoiceMutation.isPending) {
      invoiceForm.reset();
      onClose();
    }
  };

  console.log(invoiceForm.watch("shouldSendInvoiceByEmail"));

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      <DialogContent
        className="sm:max-w-md"
        onInteractOutside={(e) => e.preventDefault()}
      >
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            {t("invoice.modal.title")}
          </DialogTitle>
          <DialogDescription>
            {t("invoice.modal.description")}
          </DialogDescription>
        </DialogHeader>

        <FormProvider
          methods={invoiceForm}
          onSubmit={onSubmit}
          className="space-y-4"
        >
          {patient && (
            <div className="rounded-lg border bg-muted/50 p-3">
              <p className="text-sm font-medium text-foreground">
                {t("invoice.modal.patient")}: {patient.first_name}{" "}
                {patient.last_name}
              </p>
              <p className="text-xs text-muted-foreground">
                {t("invoice.modal.id")}: {patient.id}
              </p>
            </div>
          )}

          {!currentUser?.business_information && (
            <div className="rounded-lg border-destructive border-2 bg-muted/50 p-3">
              <div className="flex gap-2">
                <CircleAlert className="text-destructive size-8" />
                <p className="text-sm font-medium text-foreground">
                  {t("invoice.modal.missingInformationAlert")}
                </p>
              </div>
            </div>
          )}

          <div className="space-y-2">
            <Label htmlFor="amount">{t("invoice.modal.amount")} (€)</Label>
            <div className="relative">
              <FormInput
                id="amount"
                name="amount"
                type="number"
                step="0.01"
                min="0"
                placeholder="60.00"
                disabled={generateInvoiceMutation.isPending}
                className="pr-8"
              />
              <div className="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                <span className="text-muted-foreground text-sm">€</span>
              </div>
            </div>
          </div>

          <FormDatePicker
            name="date"
            label={t("invoice.modal.date")}
            disabled={generateInvoiceMutation.isPending}
          />

          <div className="py-2">
            <div className="flex items-center space-x-3">
              <FormSwitch
                id="shouldSendInvoiceByEmail"
                name="shouldSendInvoiceByEmail"
                size="lg"
                className="cursor-pointer"
              />
              <Label
                htmlFor="shouldSendInvoiceByEmail"
                className="cursor-pointer"
              >
                {t("invoice.modal.sendInvoiceByEmail")}
              </Label>
            </div>
          </div>

          <DialogFooter className="flex-col-reverse sm:flex-row gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={handleClose}
              disabled={generateInvoiceMutation.isPending}
            >
              {t("common.cancel")}
            </Button>
            {currentUser?.business_information ? (
              <Button
                type="submit"
                disabled={generateInvoiceMutation.isPending}
                className="w-full sm:w-auto"
              >
                {generateInvoiceMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin" />
                    {t("invoice.modal.generating")}
                  </>
                ) : (
                  <>
                    <FileText className="h-4 w-4" />
                    {t("invoice.modal.generate")}
                  </>
                )}
              </Button>
            ) : (
              <Button
                type="button"
                onClick={() => navigate({ to: "/my_information" })}
              >
                {t("invoice.modal.completeInformation")}
              </Button>
            )}
          </DialogFooter>
        </FormProvider>
      </DialogContent>
    </Dialog>
  );
};
