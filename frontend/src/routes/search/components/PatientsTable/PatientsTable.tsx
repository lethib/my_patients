import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Table, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { PatientList } from "./PatientsList";

interface Props {
  searchQuery: string;
}

export const PatientsTable = ({ searchQuery }: Props) => {
  const { t } = useTranslation();
  const [page, _setPage] = useState(1);

  return (
    <div className="space-y-4">
      <div className="rounded-lg border bg-card">
        <Table>
          <TableHeader>
            <TableRow className="border-b bg-muted/50">
              <TableHead className="h-12 px-6 font-semibold text-foreground">
                {t("patients.table.name")}
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                {t("patients.table.ssn")}
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                {t("patients.table.address")}
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                {t("patients.table.zip_code")}
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground">
                {t("patients.table.city")}
              </TableHead>
              <TableHead className="h-12 px-4 font-semibold text-foreground text-right">
                {t("patients.table.office")}
              </TableHead>
            </TableRow>
          </TableHeader>
          <PatientList searchQuery={searchQuery} page={page} />
        </Table>
      </div>
    </div>
  );
};
