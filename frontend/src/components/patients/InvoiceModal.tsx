import { FileText, Loader2 } from "lucide-react";
import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { patientSchema, type SearchPatientResponse } from "@/api/hooks/patient";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

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
  const [amount, setAmount] = useState("");
  const [error, setError] = useState("");

  const generateInvoiceMutation = patientSchema.generateInvoice.useMutation();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    // Validate amount
    const numericAmount = parseFloat(amount);
    if (!amount || isNaN(numericAmount) || numericAmount <= 0) {
      setError(t("invoice.errors.invalidAmount"));
      return;
    }

    generateInvoiceMutation
      .mutateAsync({
        patientId: patient.id,
        amount: `${numericAmount}€`,
      })
      .then((blob) => {
        const url = window.URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = `invoice-${patient.first_name}-${patient.last_name}-${new Date().toISOString().split("T")[0]}.pdf`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        window.URL.revokeObjectURL(url);
      });
  };

  const handleClose = () => {
    if (!generateInvoiceMutation.isPending) {
      setAmount("");
      setError("");
      onClose();
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            {t("invoice.modal.title")}
          </DialogTitle>
          <DialogDescription>
            {t("invoice.modal.description")}
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
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

          <div className="space-y-2">
            <Label htmlFor="amount">{t("invoice.modal.amount")} (€)</Label>
            <div className="relative">
              <Input
                id="amount"
                type="number"
                step="0.01"
                min="0"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="60.00"
                disabled={generateInvoiceMutation.isPending}
                required
                className="pr-8"
              />
              <div className="absolute inset-y-0 right-0 flex items-center pr-3">
                <span className="text-muted-foreground text-sm">€</span>
              </div>
            </div>
          </div>

          {error && (
            <div className="rounded-md border border-destructive/20 bg-destructive/10 p-3">
              <p className="text-sm text-destructive">{error}</p>
            </div>
          )}

          <DialogFooter className="flex-col-reverse sm:flex-row gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={handleClose}
              disabled={generateInvoiceMutation.isPending}
            >
              {t("common.cancel")}
            </Button>
            <Button
              type="submit"
              disabled={generateInvoiceMutation.isPending || !amount}
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
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
