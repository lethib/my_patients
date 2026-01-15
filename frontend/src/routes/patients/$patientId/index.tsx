import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { ArrowLeft, Calendar } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui";
import { H2 } from "@/components/ui/typography/h2";
import { AppointmentsTable } from "./components/AppointmentsTable/AppointmentsTable";
import { PatientInformationCard } from "./components/PatientInformationCard";

export const Route = createFileRoute("/patients/$patientId/")({
  component: PatientPage,
});

function PatientPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { patientId } = Route.useParams();

  return (
    <div className="min-h-screen bg-gradient-to-br from-background via-background to-muted/20">
      <div className="container mx-auto px-4 py-8 space-y-6">
        <Button
          variant="link"
          onClick={() => navigate({ to: "/patients" })}
          className="flex items-center gap-2"
        >
          <ArrowLeft className="h-4 w-4" />
          {t("common.backToPatients")}
        </Button>

        <PatientInformationCard patientId={+patientId} />

        <div className="space-y-4">
          <div className="flex justify-between items-center">
            <div>
              <H2 className="text-2xl font-bold flex items-center gap-2">
                <Calendar className="h-6 w-6" />
                {t("appointments.title")}
              </H2>
              <p className="text-muted-foreground text-sm mt-1">
                {t("appointments.subtitle")}
              </p>
            </div>
          </div>
        </div>

        <AppointmentsTable patientId={+patientId} />
      </div>
    </div>
  );
}
