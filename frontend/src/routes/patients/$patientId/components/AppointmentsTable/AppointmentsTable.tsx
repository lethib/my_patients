import { Calendar, Euro, MapPin } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Table, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { AppointmentsList } from "./AppointmentsList";

interface Props {
  patientId: number;
}

export const AppointmentsTable = ({ patientId }: Props) => {
  const { t } = useTranslation();
  return (
    <div className="rounded-lg border bg-card">
      <Table>
        <TableHeader>
          <TableRow className="border-b bg-muted/50">
            <TableHead className="h-12 px-6 font-semibold text-foreground">
              <div className="flex items-center gap-2">
                <Calendar className="h-4 w-4" />
                {t("appointments.table.date")}
              </div>
            </TableHead>
            <TableHead className="h-12 px-4 font-semibold text-foreground">
              <div className="flex items-center gap-2">
                <MapPin className="h-4 w-4" />
                {t("appointments.table.office")}
              </div>
            </TableHead>
            <TableHead className="h-12 px-4 font-semibold text-foreground">
              <div className="flex items-center gap-2">
                <Euro className="h-4 w-4" />
                {t("appointments.table.price")}
              </div>
            </TableHead>
          </TableRow>
        </TableHeader>

        <AppointmentsList patientId={patientId} />
      </Table>
    </div>
  );
};
