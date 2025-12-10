import { useTranslation } from "react-i18next";
import { queryClient } from "@/api/api";
import { APIHooks } from "@/api/hooks";
import type { SearchPatientResponse } from "@/api/hooks/patient";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui";

interface ConfirmPatientDeleteProps {
  isOpen: boolean;
  onClose: () => void;
  patient: SearchPatientResponse;
}

export const ConfirmPatientDeleteModal = ({
  isOpen,
  onClose,
  patient,
}: ConfirmPatientDeleteProps) => {
  const { t } = useTranslation();

  const deletePatientMutation = APIHooks.patient.deletePatient.useMutation({
    patient_id: patient.id,
  });

  const deletePatient = () => {
    deletePatientMutation.mutateAsync(null).then(() => {
      queryClient.invalidateQueries({ queryKey: ["/patient/_search"] });
      onClose();
    });
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Suppression du patient</DialogTitle>
          <DialogDescription>
            La suppression du patient entraîne la perte de toutes les
            consultations avec {patient.first_name} {patient.last_name}.
            Voulez-vous continuez?
          </DialogDescription>
        </DialogHeader>

        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>
            {t("common.close")}
          </Button>
          <Button
            type="button"
            variant="destructive"
            onClick={deletePatient}
            className="w-full sm:w-auto"
          >
            {t("common.delete")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
