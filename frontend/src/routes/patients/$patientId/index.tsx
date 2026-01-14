import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { ArrowLeft, Calendar, Edit, Plus, User } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { APIHooks } from "@/api/hooks";
import type { MedicalAppointment } from "@/api/hooks/appointment";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { CenteredSpineer } from "@/components/ui/spinner";
import { H2 } from "@/components/ui/typography/h2";
import { PatientModal } from "@/routes/search/components/PatientModal/PatientModal";
import { AppointmentModal } from "./components/AppointmentModal";
import { AppointmentsTable } from "./components/AppointmentsTable";
import { ConfirmAppointmentDeleteModal } from "./components/ConfirmAppointmentDeleteModal";

export const Route = createFileRoute("/patients/$patientId/")({
  component: PatientAppointments,
});

function PatientAppointments() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { patientId } = Route.useParams();
  const numericPatientId = Number(patientId);

  const [isEditPatientModalOpen, setIsEditPatientModalOpen] = useState(false);
  const [isAppointmentModalOpen, setIsAppointmentModalOpen] = useState(false);
  const [selectedAppointment, setSelectedAppointment] =
    useState<MedicalAppointment | null>(null);
  const [appointmentToDelete, setAppointmentToDelete] =
    useState<MedicalAppointment | null>(null);

  // Fetch patient data using mock endpoint
  const patientQuery = APIHooks.patient.getById.useQuery(numericPatientId);

  const appointmentsQuery =
    APIHooks.appointment.getByPatientId.useQuery(numericPatientId);

  const updatePatientMutation = APIHooks.patient.updatePatient.useMutation({
    patient_id: numericPatientId,
  });

  const patient = patientQuery.data;

  const handleEditAppointment = (appointment: MedicalAppointment) => {
    setSelectedAppointment(appointment);
    setIsAppointmentModalOpen(true);
  };

  const handleCreateAppointment = () => {
    setSelectedAppointment(null);
    setIsAppointmentModalOpen(true);
  };

  const handleCloseAppointmentModal = () => {
    setIsAppointmentModalOpen(false);
    setSelectedAppointment(null);
  };

  if (patientQuery.isLoading || !patient) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-background via-background to-muted/20">
        <div className="container mx-auto px-4 py-8">
          <CenteredSpineer />
        </div>
      </div>
    );
  }

  const formattedSSN = `${patient.ssn[0]} ${patient.ssn.slice(1, 3)} ${patient.ssn.slice(3, 5)} ${patient.ssn.slice(5, 7)} ${patient.ssn.slice(7, 10)} ${patient.ssn.slice(10, 13)} ${patient.ssn.slice(13, 15)}`;

  return (
    <>
      <div className="min-h-screen bg-gradient-to-br from-background via-background to-muted/20">
        <div className="container mx-auto px-4 py-8 space-y-6">
          {/* Back Navigation */}
          <Button
            variant="ghost"
            onClick={() => navigate({ to: "/search" })}
            className="flex items-center gap-2"
          >
            <ArrowLeft className="h-4 w-4" />
            {t("common.backToPatients", "Retour aux patients")}
          </Button>

          {/* Patient Info Card */}
          <Card>
            <CardHeader>
              <div className="flex justify-between items-start">
                <div className="flex items-center gap-4">
                  <div className="flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
                    <User className="h-8 w-8 text-primary" />
                  </div>
                  <div>
                    <CardTitle className="text-2xl">
                      {patient.first_name} {patient.last_name}
                    </CardTitle>
                    <CardDescription className="flex flex-col gap-1 mt-2">
                      <span>SSN: {formattedSSN}</span>
                      <span>
                        {patient.address_line_1}, {patient.address_zip_code}{" "}
                        {patient.address_city}
                      </span>
                      {patient.email && <span>Email: {patient.email}</span>}
                    </CardDescription>
                  </div>
                </div>
                <Button
                  variant="outline"
                  onClick={() => setIsEditPatientModalOpen(true)}
                  className="flex items-center gap-2"
                >
                  <Edit className="h-4 w-4" />
                  {t("common.edit", "Modifier")}
                </Button>
              </div>
            </CardHeader>
          </Card>

          {/* Appointments Section */}
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <div>
                <H2 className="text-2xl font-bold flex items-center gap-2">
                  <Calendar className="h-6 w-6" />
                  {t("appointments.title", "Rendez-vous")}
                </H2>
                <p className="text-muted-foreground text-sm mt-1">
                  {t(
                    "appointments.subtitle",
                    "Gérer les rendez-vous médicaux de ce patient"
                  )}
                </p>
              </div>
              <Button
                onClick={handleCreateAppointment}
                className="flex items-center gap-2"
              >
                <Plus className="h-4 w-4" />
                {t("appointments.addAppointment", "Ajouter un rendez-vous")}
              </Button>
            </div>

            <AppointmentsTable
              appointments={appointmentsQuery.data || []}
              isLoading={appointmentsQuery.isLoading}
              onEdit={handleEditAppointment}
              onDelete={setAppointmentToDelete}
            />
          </div>
        </div>
      </div>

      {/* Patient Edit Modal */}
      <PatientModal
        open={isEditPatientModalOpen}
        asyncMutation={updatePatientMutation.mutateAsync}
        onOpenChange={setIsEditPatientModalOpen}
        selectedPatient={patient}
      />

      {/* Appointment Create/Edit Modal */}
      <AppointmentModal
        open={isAppointmentModalOpen}
        onOpenChange={handleCloseAppointmentModal}
        patientId={numericPatientId}
        selectedAppointment={selectedAppointment}
      />

      {/* Delete Confirmation Modal */}
      {appointmentToDelete && (
        <ConfirmAppointmentDeleteModal
          isOpen={!!appointmentToDelete}
          onClose={() => setAppointmentToDelete(null)}
          appointment={appointmentToDelete}
          patientId={numericPatientId}
        />
      )}
    </>
  );
}
