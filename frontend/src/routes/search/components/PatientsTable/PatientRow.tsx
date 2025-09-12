import { User } from "lucide-react";
import type { SearchPatientResponse } from "@/api/hooks/patient";
import { TableCell, TableRow } from "@/components/ui/table";

interface Props {
  patient: SearchPatientResponse;
  index: number;
}

export const PatientRow = ({ patient, index }: Props) => {
  const formattedSSN = `${patient.ssn[0]} ${patient.ssn.slice(1, 3)} ${patient.ssn.slice(3, 5)} ${patient.ssn.slice(5, 7)} ${patient.ssn.slice(7, 10)} ${patient.ssn.slice(10, 13)} ${patient.ssn.slice(13, 15)}`;

  return (
    <TableRow
      className={`cursor-pointer transition-colors hover:bg-muted/30 ${
        index % 2 === 0 ? "bg-background" : "bg-muted/10"
      }`}
    >
      <TableCell className="px-6 py-4">
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
            <User className="h-5 w-5 text-primary" />
          </div>
          <div className="flex flex-col">
            <span className="font-semibold text-foreground">
              {patient.first_name} {patient.last_name}
            </span>
            <span className="text-xs text-muted-foreground">
              ID: {patient.id}
            </span>
          </div>
        </div>
      </TableCell>
      <TableCell className="px-4 py-4">
        <span className="font-mono text-sm font-medium">{formattedSSN}</span>
      </TableCell>
      <TableCell className="px-4 py-4 text-muted-foreground">
        {patient.address_line_1}
      </TableCell>
      <TableCell className="px-4 py-4 text-muted-foreground">
        <span className="font-mono text-sm">{patient.address_zip_code}</span>
      </TableCell>
      <TableCell className="px-4 py-4 text-muted-foreground">
        <span className="text-sm">{patient.address_city}</span>
      </TableCell>
      <TableCell className="px-4 py-4 text-right">
        <div className="flex flex-col items-end">
          <span className="text-sm font-semibold text-primary">
            {new Date("2025-06-25").toLocaleDateString("en-US", {
              month: "short",
              day: "numeric",
              year: "numeric",
            })}
          </span>
          <span className="text-xs text-muted-foreground">
            {Math.floor(
              (new Date().getTime() - new Date("2025-06-25").getTime()) /
                (1000 * 60 * 60 * 24),
            )}{" "}
            days ago
          </span>
        </div>
      </TableCell>
    </TableRow>
  );
};
