import { useState } from "react";
import { Table, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { PatientList } from "./PatientsList";

interface Props {
  searchQuery: string;
}

export const PatientsTable = ({ searchQuery }: Props) => {
  const [page, _setPage] = useState(1);

  return (
    <div className="space-y-4">
      <div className="rounded-lg border bg-card">
        <Table>
          <TableHeader>
            <TableRow className="border-b bg-muted/50">
              <TableHead className="h-12 px-6 font-semibold text-foreground">
                Name
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                SSN
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                Address
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                Zip Code
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                City
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground text-right">
                Last Visited
              </TableHead>
            </TableRow>
          </TableHeader>
          <PatientList searchQuery={searchQuery} page={page} />
        </Table>
      </div>
    </div>
  );
};
