import { useState } from "react";
import type { SearchPatientResponse } from "@/api/hooks/patient";
import { CenteredSpineer } from "@/components/ui/spinner";
import { TableBody, TableCell, TableRow } from "@/components/ui/table";
import { ConfirmPatientDeleteModal } from "../ConfirmPatientDeleteModal";
import { InvoiceModal } from "../InvoiceModal/InvoiceModal";
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
  const [patientToUpdate, setPatientToUpdate] =
    useState<SearchPatientResponse | null>(null);
  const [patientToDelete, setPatientToDelete] =
    useState<SearchPatientResponse | null>(null);
  const [isInvoiceModalOpen, setIsInvoiceModalOpen] = useState(false);
  const [isConfirmDeletePatientModalOpen, setIsConfirmDeletePatientModalOpen] =
    useState(false);

  const handleGenerateInvoice = (patient: SearchPatientResponse) => {
    setPatientToUpdate(patient);
    setIsInvoiceModalOpen(true);
  };

  const handleCloseInvoiceModal = () => {
    setIsInvoiceModalOpen(false);
    setPatientToUpdate(null);
  };

  const handleDeletePatient = (patient: SearchPatientResponse) => {
    setPatientToDelete(patient);
    setIsConfirmDeletePatientModalOpen(true);
  };

  const handleCloseConfirmPatientDelete = () => {
    setIsConfirmDeletePatientModalOpen(false);
    setPatientToDelete(null);
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
            onDeletePatient={handleDeletePatient}
            onClickRow={onClickRow}
          />
        ))}
      </TableBody>

      {patientToUpdate && (
        <InvoiceModal
          isOpen={isInvoiceModalOpen}
          onClose={handleCloseInvoiceModal}
          patient={patientToUpdate}
        />
      )}

      {patientToDelete && (
        <ConfirmPatientDeleteModal
          isOpen={isConfirmDeletePatientModalOpen}
          onClose={handleCloseConfirmPatientDelete}
          patient={patientToDelete}
        />
      )}
    </>
  );
};
