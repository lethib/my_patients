import { useState } from "react";
import { APIHooks } from "@/api/hooks";
import type { SearchPatientResponse } from "@/api/hooks/patient";
import { InvoiceModal } from "@/components/patients/InvoiceModal";
import { CenteredSpineer } from "@/components/ui/spinner";
import { TableBody, TableCell, TableRow } from "@/components/ui/table";
import { PatientRow } from "./PatientRow";

interface Props {
  searchQuery: string;
  page: number;
  onClickRow: (patient: SearchPatientResponse) => void;
}

export const PatientList = ({ searchQuery, page, onClickRow }: Props) => {
  const [selectedPatient, setSelectedPatient] =
    useState<SearchPatientResponse | null>(null);
  const [isInvoiceModalOpen, setIsInvoiceModalOpen] = useState(false);

  const searchPatientsQuery = APIHooks.patient.search.useQuery({
    q: searchQuery,
    page,
  });

  const handleGenerateInvoice = (patient: SearchPatientResponse) => {
    setSelectedPatient(patient);
    setIsInvoiceModalOpen(true);
  };

  const handleCloseInvoiceModal = () => {
    setIsInvoiceModalOpen(false);
    setSelectedPatient(null);
  };

  if (searchPatientsQuery.data?.paginated_data.length === 0) {
    return null; // Let parent handle empty state
  }

  if (searchPatientsQuery.isFetching) {
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
        {searchPatientsQuery.data?.paginated_data.map((patient, index) => (
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
