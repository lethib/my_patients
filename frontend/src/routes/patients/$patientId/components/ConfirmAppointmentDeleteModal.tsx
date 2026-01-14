import { useTranslation } from "react-i18next";
import { APIHooks } from "@/api/hooks";
import type { MedicalAppointment } from "@/api/hooks/appointment";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui";

interface Props {
  isOpen: boolean;
  onClose: () => void;
  appointment: MedicalAppointment;
  patientId: number;
}

export const ConfirmAppointmentDeleteModal = ({
  isOpen,
  onClose,
  appointment,
  patientId,
}: Props) => {
  const { t } = useTranslation();
  const deleteMutation = APIHooks.appointment.delete.useMutation();

  const handleDelete = async () => {
    try {
      await deleteMutation.mutateAsync({
        id: appointment.id,
        patient_id: patientId,
      });
      onClose();
    } catch (error) {
      console.error("Failed to delete appointment:", error);
    }
  };

  const formattedDate = new Date(appointment.date).toLocaleDateString();
  const formattedPrice = (appointment.price_in_cents / 100).toFixed(2);

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {t(
              "appointments.deleteModal.title",
              "Supprimer le rendez-vous"
            )}
          </DialogTitle>
          <DialogDescription>
            {t(
              "appointments.deleteModal.description",
              "Êtes-vous sûr de vouloir supprimer ce rendez-vous ? Cette action est irréversible."
            )}
          </DialogDescription>
        </DialogHeader>

        <div className="rounded-lg border bg-muted/50 p-3">
          <p className="text-sm">
            <span className="font-medium">Date :</span> {formattedDate}
          </p>
          <p className="text-sm">
            <span className="font-medium">Prix :</span> {formattedPrice} €
          </p>
        </div>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={onClose}
            disabled={deleteMutation.isPending}
          >
            {t("common.cancel", "Annuler")}
          </Button>
          <Button
            type="button"
            variant="destructive"
            onClick={handleDelete}
            disabled={deleteMutation.isPending}
          >
            {t("common.delete", "Supprimer")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
