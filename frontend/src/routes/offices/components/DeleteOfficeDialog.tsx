import { useTranslation } from "react-i18next";
import { APIClient, queryClient } from "@/api/api";
import type { PractitionerOffice } from "@/api/hooks/practitioner_office";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui";

interface DeleteOfficeDialogProps {
  open: boolean;
  setIsOpen: (open: boolean) => void;
  office: PractitionerOffice;
}

export const DeleteOfficeDialog = ({
  open,
  setIsOpen,
  office,
}: DeleteOfficeDialogProps) => {
  const { t } = useTranslation();

  const deleteOfficeMutation = APIClient.hooks.office.deleteOffice.useMutation({
    office_id: office.id,
  });

  const handleDelete = () =>
    deleteOfficeMutation.mutateAsync(null).then(() => {
      queryClient.invalidateQueries({ queryKey: ["/user/my_offices"] });
      setIsOpen(false);
    });

  return (
    <Dialog open={open} onOpenChange={setIsOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("offices.delete.title")}</DialogTitle>
          <DialogDescription>
            {t("offices.delete.description")}
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" onClick={() => setIsOpen(false)}>
            {t("common.cancel")}
          </Button>
          <Button variant="destructive" onClick={handleDelete}>
            {t("common.delete")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
