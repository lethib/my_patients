import { createFileRoute } from "@tanstack/react-router";
import { Plus, Search as SearchIcon } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { H2 } from "@/components/ui/typography/h2";
import { useDebounce } from "@/hooks/useDebounce";
import { AddPatientModal } from "./components/AddPatientModal";
import { PatientsTable } from "./components/PatientsTable/PatientsTable";

export const Route = createFileRoute("/search/")({
  component: Search,
});

function Search() {
  const [isAddPatientModalOpened, setIsAddPatientModalOpened] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const debouncedSearchQuery = useDebounce(searchQuery, 700);

  return (
    <>
      <div className="min-h-screen bg-gradient-to-br from-background via-background to-muted/20">
        <div className="container mx-auto px-4 py-8">
          {/* Header */}
          <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-8">
            <div>
              <H2 className="text-3xl font-bold mb-2">Patients</H2>
              <p className="text-muted-foreground">
                Manage and search through your patient records
              </p>
            </div>
            <Button
              onClick={() => setIsAddPatientModalOpened(true)}
              className="flex items-center gap-2"
              size="lg"
            >
              <Plus className="h-4 w-4" />
              Add Patient
            </Button>
          </div>

          {/* Search Bar */}
          <div className="relative mb-8">
            <SearchIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="Search patients by name or SSN..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10 h-12 text-base"
            />
          </div>

          <PatientsTable searchQuery={debouncedSearchQuery} />
        </div>
      </div>

      <AddPatientModal
        open={isAddPatientModalOpened}
        setIsOpen={() => setIsAddPatientModalOpened(false)}
      />
    </>
  );
}
