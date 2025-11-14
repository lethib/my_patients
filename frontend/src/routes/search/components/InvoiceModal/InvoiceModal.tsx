import React, { useState } from "react";
import { type SearchPatientResponse } from "@/api/hooks/patient";
import { Dialog } from "@/components/ui/dialog";
import { DownloadGeneratedInvoiceContent } from "./DownloadGeneratedInvoiceContent";
import { GenerateInvoiceContent } from "./GenerateInvoiceContent";

interface InvoiceModalProps {
  isOpen: boolean;
  onClose: () => void;
  patient: SearchPatientResponse;
}

export type LocalInvoiceFile = { blob: Blob; filename: string };

export const InvoiceModal: React.FC<InvoiceModalProps> = ({
  isOpen,
  onClose,
  patient,
}) => {
  const [generatedInvoice, setGeneratedInvoice] =
    useState<LocalInvoiceFile | null>(null);
  const [isEmailSent, setIsEmailSent] = useState(false);

  const handleClose = () => {
    setGeneratedInvoice(null);
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      {generatedInvoice ? (
        <DownloadGeneratedInvoiceContent
          generatedInvoice={generatedInvoice}
          isEmailSent={isEmailSent}
          handleClose={handleClose}
        />
      ) : (
        <GenerateInvoiceContent
          patient={patient}
          onClose={handleClose}
          setGeneratedInvoice={setGeneratedInvoice}
          setIsEmailSent={setIsEmailSent}
        />
      )}
    </Dialog>
  );
};
