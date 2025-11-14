import { CheckCircle2, Download, FileText } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui";
import {
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import type { LocalInvoiceFile } from "./InvoiceModal";

interface DownloadGeneratedInvoiceModal {
  generatedInvoice: LocalInvoiceFile;
  isEmailSent: boolean;
  handleClose: VoidFunction;
}

export const DownloadGeneratedInvoiceContent = ({
  generatedInvoice,
  isEmailSent,
  handleClose,
}: DownloadGeneratedInvoiceModal) => {
  const { t } = useTranslation();

  const handleDownload = () => {
    const url = window.URL.createObjectURL(generatedInvoice.blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = generatedInvoice.filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    window.URL.revokeObjectURL(url);
  };

  return (
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
          {t("invoice.modal.downloadDescription")}
        </DialogDescription>
      </DialogHeader>

      <div className="space-y-4">
        {isEmailSent && (
          <div className="rounded-lg border-green-500 border-2 bg-green-50 dark:bg-green-950/20 p-3">
            <div className="flex gap-2 items-center">
              <CheckCircle2 className="text-green-600 dark:text-green-400 size-5 flex-shrink-0" />
              <p className="text-sm font-medium text-green-900 dark:text-green-100">
                {t("invoice.modal.emailSentConfirmation")}
              </p>
            </div>
          </div>
        )}

        <div className="rounded-lg border bg-muted/50 p-3">
          <p className="text-sm text-muted-foreground mt-1">
            {t("invoice.modal.filename")}: {generatedInvoice.filename}
          </p>
        </div>

        <DialogFooter className="flex-col-reverse sm:flex-row gap-2">
          <Button type="button" variant="outline" onClick={handleClose}>
            {t("common.close")}
          </Button>
          <Button
            type="button"
            onClick={handleDownload}
            className="w-full sm:w-auto"
          >
            <Download className="h-4 w-4" />
            {t("invoice.modal.download")}
          </Button>
        </DialogFooter>
      </div>
    </DialogContent>
  );
};
