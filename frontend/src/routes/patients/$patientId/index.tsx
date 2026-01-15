import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui";
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
      </div>
    </div>
  );
}
