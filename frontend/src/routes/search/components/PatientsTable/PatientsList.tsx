import { useState } from "react";
import type { SearchPatientResponse } from "@/api/hooks/patient";
import { InvoiceModal } from "@/components/patients/InvoiceModal";
import { CenteredSpineer } from "@/components/ui/spinner";
import { TableBody, TableCell, TableRow } from "@/components/ui/table";
import { PatientRow } from "./PatientRow";

interface Props {
  patientsList: SearchPatientResponse[] | undefined;
  isDataFetching: boolean;
  onClickRow: (patient: SearchPatientResponse) => void;
}

export const PatientList = ({
  patientsList,
  isDataFetching,
  onClickRow,
}: Props) => {
  const [selectedPatient, setSelectedPatient] =
    useState<SearchPatientResponse | null>(null);
  const [isInvoiceModalOpen, setIsInvoiceModalOpen] = useState(false);

  const handleGenerateInvoice = (patient: SearchPatientResponse) => {
    setSelectedPatient(patient);
    setIsInvoiceModalOpen(true);
  };

  const handleCloseInvoiceModal = () => {
    setIsInvoiceModalOpen(false);
    setSelectedPatient(null);
  };

  if (patientsList?.length === 0) {
    return null; // Let parent handle empty state
  }

  if (isDataFetching) {
    return (
      <TableBody>
        <TableRow>
          <TableCell colSpan={7} className="h-32 text-center">
            <CenteredSpineer />
          </TableCell>
        </TableRow>
      </TableBody>
    );
  }

  return (
    <>
      <TableBody>
        {patientsList?.map((patient, index) => (
          <PatientRow
            patient={patient}
            index={index}
            key={patient.id}
            onGenerateInvoice={handleGenerateInvoice}
            onClickRow={onClickRow}
          />
        ))}
      </TableBody>

      {selectedPatient && (
        <InvoiceModal
          isOpen={isInvoiceModalOpen}
          onClose={handleCloseInvoiceModal}
          patient={selectedPatient}
        />
      )}
    </>
  );
};
