import { Plus, User } from "lucide-react";
import { useTranslation } from "react-i18next";
import type {
  SearchBySSNPatientResponse,
  SearchPatientResponse,
} from "@/api/hooks/patient";
import { Card } from "@/components/ui";

interface PatientSelectorProps {
  patients: SearchBySSNPatientResponse[];
  onSelectExistingPatient: (patient: SearchPatientResponse) => void;
  onCreateNewPatient: () => void;
}

export const PatientSelector = ({
  patients,
  onSelectExistingPatient,
  onCreateNewPatient,
}: PatientSelectorProps) => {
  const { t } = useTranslation();

  return (
    <div className="flex flex-col gap-2">
      <span className="text-sm">{t("patients.form.selector.prompt")}</span>
      {patients.map((patient) => (
        <Card
          key={patient.id}
          className="py-2 pl-5 cursor-pointer hover:bg-accent transition-colors"
          onClick={() => onSelectExistingPatient(patient)}
        >
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
              <User className="h-5 w-5 text-primary" />
            </div>
            <div className="flex flex-col">
              <span className="text-foreground text-sm font-semibold">
                {patient.first_name} {patient.last_name}
              </span>
              <span className="text-xs text-muted-foreground">
                {t("patients.form.selector.patientId")}: {patient.id}
              </span>
            </div>
          </div>
        </Card>
      ))}
      <Card
        className="py-2 pl-5 cursor-pointer hover:bg-accent transition-colors"
        onClick={onCreateNewPatient}
      >
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
            <Plus className="h-5 w-5 text-primary" />
          </div>
          <div className="flex flex-col">
            <span className="text-foreground text-sm font-semibold">
              {t("patients.form.selector.newPatient")}
            </span>
          </div>
        </div>
      </Card>
    </div>
  );
};
