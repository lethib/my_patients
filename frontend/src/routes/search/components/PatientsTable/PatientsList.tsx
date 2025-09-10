import { APIHooks } from "@/api/hooks";
import { CenteredSpineer } from "@/components/ui/spinner";
import { TableBody, TableCell, TableRow } from "@/components/ui/table";
import { PatientRow } from "./PatientRow";

interface Props {
  searchQuery: string;
  page: number;
}

export const PatientList = ({ searchQuery, page }: Props) => {
  const searchPatientsQuery = APIHooks.patient.search.useQuery({
    q: searchQuery,
    page,
  });

  if (searchPatientsQuery.data?.paginated_data.length === 0) {
    return null; // Let parent handle empty state
  }

  if (searchPatientsQuery.isFetching) {
    return (
      <TableBody>
        <TableRow>
          <TableCell colSpan={6} className="h-32 text-center">
            <CenteredSpineer />
          </TableCell>
        </TableRow>
      </TableBody>
    );
  }

  return (
    <TableBody>
      {searchPatientsQuery.data?.paginated_data.map((patient, index) => (
        <PatientRow patient={patient} index={index} />
      ))}
    </TableBody>
  );
};
