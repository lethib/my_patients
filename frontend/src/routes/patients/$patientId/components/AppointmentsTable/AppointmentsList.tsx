import { Edit } from "lucide-react";
import { APIHooks } from "@/api/hooks";
import type { MedicalAppointment } from "@/api/hooks/patient";
import { Button } from "@/components/ui";
import { CenteredSpineer } from "@/components/ui/spinner";
import { TableBody, TableCell, TableRow } from "@/components/ui/table";

interface Props {
  patientId: number;
  onClickEditAppointment: (appointment: MedicalAppointment) => void;
}

export const AppointmentsList = ({
  patientId,
  onClickEditAppointment,
}: Props) => {
  const medicalAppointmentsQuery = APIHooks.patient
    .getMedicalAppointments(patientId)
    .useQuery(null);

  if (medicalAppointmentsQuery.isFetching) {
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
    <TableBody>
      {medicalAppointmentsQuery.data?.map((appointment, index) => (
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
            {appointment.office.name}
          </TableCell>
          <TableCell className="px-4 py-4">
            <span className="font-mono font-medium">
              {(appointment.price_in_cents / 100).toFixed(2)} €
            </span>
          </TableCell>
          <TableCell>
            <Button
              variant="outline"
              size="sm"
              className="h-8 w-8 p-0"
              onClick={(e) => {
                e.stopPropagation();
                onClickEditAppointment(appointment);
              }}
            >
              <Edit className="h-4 w-4" />
            </Button>
          </TableCell>
        </TableRow>
      ))}
    </TableBody>
  );
};
