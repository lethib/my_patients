import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { LogOut, Plus, Search as SearchIcon, UserCog } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { H2 } from "@/components/ui/typography/h2";
import { useDebounce } from "@/hooks/useDebounce";
import { logout } from "@/lib/authUtils";
import { AddPatientModal } from "./components/AddPatientModal";
import { PatientsTable } from "./components/PatientsTable/PatientsTable";

export const Route = createFileRoute("/search/")({
  component: Search,
});

function Search() {
  const { t } = useTranslation();
  const [isAddPatientModalOpened, setIsAddPatientModalOpened] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const debouncedSearchQuery = useDebounce(searchQuery, 700);
  const navigate = useNavigate();

  return (
    <>
      <div className="min-h-screen bg-gradient-to-br from-background via-background to-muted/20">
        <div className="container mx-auto px-4 py-8">
          {/* Header */}
          <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-8">
            <div>
              <H2 className="text-3xl font-bold mb-2">{t("patients.title")}</H2>
              <p className="text-muted-foreground">{t("patients.subtitle")}</p>
            </div>
            <div className="flex gap-4">
              <Button
                onClick={() => setIsAddPatientModalOpened(true)}
                className="flex items-center gap-2"
              >
                <Plus className="h-4 w-4" />
                {t("patients.addPatient")}
              </Button>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost">
                    <UserCog className="size-6 text-primary" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent side="bottom" align="end">
                  <DropdownMenuLabel>
                    {t("navigation.account")}
                  </DropdownMenuLabel>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    onClick={() => navigate({ to: "/my_information" })}
                  >
                    {t("navigation.myInformation")}
                  </DropdownMenuItem>
                  <DropdownMenuItem onClick={logout} variant="destructive">
                    {t("auth.logout")} <LogOut />
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>

          {/* Search Bar */}
          <div className="relative mb-8">
            <SearchIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder={t("patients.searchPlaceholder")}
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
