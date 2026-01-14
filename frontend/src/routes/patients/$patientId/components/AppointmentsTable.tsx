import { Calendar, Edit, Euro, MapPin, Trash2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { APIHooks } from "@/api/hooks";
import type { MedicalAppointment } from "@/api/hooks/appointment";
import { Button } from "@/components/ui/button";
import { CenteredSpineer } from "@/components/ui/spinner";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

interface Props {
  appointments: MedicalAppointment[];
  isLoading: boolean;
  onEdit: (appointment: MedicalAppointment) => void;
  onDelete: (appointment: MedicalAppointment) => void;
}

export const AppointmentsTable = ({
  appointments,
  isLoading,
  onEdit,
  onDelete,
}: Props) => {
  const { t } = useTranslation();
  const officesQuery = APIHooks.user.getMyOffices.useQuery(null);

  // Get office name by ID
  const getOfficeName = (officeId: number) => {
    const office = officesQuery.data?.find((o) => o.id === officeId);
    return office?.name || t("appointments.unknownOffice", "Cabinet inconnu");
  };

  // Format price from cents to euros
  const formatPrice = (priceInCents: number) => {
    return (priceInCents / 100).toFixed(2);
  };

  // Sort appointments by date (newest first)
  const sortedAppointments = [...appointments].sort((a, b) => {
    return new Date(b.date).getTime() - new Date(a.date).getTime();
  });

  if (isLoading) {
    return (
      <div className="rounded-lg border bg-card p-8">
        <CenteredSpineer />
      </div>
    );
  }

  if (appointments.length === 0) {
    return (
      <div className="rounded-lg border bg-card p-12 text-center">
        <Calendar className="h-12 w-12 mx-auto mb-4 text-muted-foreground" />
        <p className="text-muted-foreground">
          {t("appointments.noAppointments", "Aucun rendez-vous enregistré.")}
        </p>
        <p className="text-sm text-muted-foreground mt-2">
          {t(
            "appointments.addFirstAppointment",
            "Cliquez sur 'Ajouter un rendez-vous' pour en créer un."
          )}
        </p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border bg-card">
      <Table>
        <TableHeader>
          <TableRow className="border-b bg-muted/50">
            <TableHead className="h-12 px-6 font-semibold text-foreground">
              <div className="flex items-center gap-2">
                <Calendar className="h-4 w-4" />
                {t("appointments.table.date", "Date")}
              </div>
            </TableHead>
            <TableHead className="h-12 px-4 font-semibold text-foreground">
              <div className="flex items-center gap-2">
                <MapPin className="h-4 w-4" />
                {t("appointments.table.office", "Cabinet")}
              </div>
            </TableHead>
            <TableHead className="h-12 px-4 font-semibold text-foreground">
              <div className="flex items-center gap-2">
                <Euro className="h-4 w-4" />
                {t("appointments.table.price", "Prix")}
              </div>
            </TableHead>
            <TableHead className="h-12 px-4 font-semibold text-foreground text-right">
              {t("common.actions", "Actions")}
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {sortedAppointments.map((appointment, index) => (
            <TableRow
              key={appointment.id}
              className={`transition-colors hover:bg-muted/30 ${
                index % 2 === 0 ? "bg-background" : "bg-muted/10"
              }`}
            >
              <TableCell className="px-6 py-4">
                <span className="font-medium">
                  {new Date(appointment.date).toLocaleDateString()}
                </span>
              </TableCell>
              <TableCell className="px-4 py-4 text-muted-foreground">
                {getOfficeName(appointment.practitioner_office_id)}
              </TableCell>
              <TableCell className="px-4 py-4">
                <span className="font-mono font-medium">
                  {formatPrice(appointment.price_in_cents)} €
                </span>
              </TableCell>
              <TableCell className="px-4 py-4 text-right space-x-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => onEdit(appointment)}
                  className="h-8 w-8 p-0"
                >
                  <Edit className="h-4 w-4" />
                </Button>
                <Button
                  variant="ghost_destructive"
                  size="sm"
                  className="h-8 w-8 p-0"
                  onClick={() => onDelete(appointment)}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};
