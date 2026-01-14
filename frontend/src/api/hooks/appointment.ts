import { useMutation, useQuery } from "@tanstack/react-query";
import { queryClient } from "../api";

// Types matching backend model
export type MedicalAppointment = {
  id: number;
  user_id: number;
  patient_id: number;
  practitioner_office_id: number;
  date: string; // ISO date string "YYYY-MM-DD"
  price_in_cents: number;
  created_at: string;
  updated_at: string;
};

export type CreateAppointmentParams = {
  patient_id: number;
  practitioner_office_id: number;
  date: string;
  price_in_cents: number;
};

export type UpdateAppointmentParams = CreateAppointmentParams;

// Mock data generator (deterministic based on patient_id)
function generateMockAppointments(patientId: number): MedicalAppointment[] {
  // Generate 3-8 appointments per patient
  const count = 3 + (patientId % 6);
  const appointments: MedicalAppointment[] = [];

  for (let i = 0; i < count; i++) {
    const daysAgo = i * 30 + (patientId % 10);
    const date = new Date();
    date.setDate(date.getDate() - daysAgo);

    appointments.push({
      id: patientId * 1000 + i,
      user_id: 1,
      patient_id: patientId,
      practitioner_office_id: 1 + (i % 3), // Rotate through offices
      date: date.toISOString().split("T")[0],
      price_in_cents: 6000 + ((i + patientId) % 5) * 500, // 60-62 euros
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    });
  }

  return appointments;
}

// In-memory store for mock mutations
const mockAppointmentsStore = new Map<number, MedicalAppointment[]>();

export const appointmentSchema = {
  getByPatientId: {
    useQuery: (patientId: number) => {
      return useQuery<MedicalAppointment[]>({
        queryKey: ["/appointment/by_patient", patientId],
        queryFn: async () => {
          // Simulate API delay
          await new Promise((resolve) => setTimeout(resolve, 300));

          // Get or generate appointments for this patient
          if (!mockAppointmentsStore.has(patientId)) {
            mockAppointmentsStore.set(
              patientId,
              generateMockAppointments(patientId)
            );
          }
          return mockAppointmentsStore.get(patientId)!;
        },
      });
    },
  },

  create: {
    useMutation: () => {
      return useMutation<MedicalAppointment, Error, CreateAppointmentParams>({
        mutationFn: async (params) => {
          await new Promise((resolve) => setTimeout(resolve, 300));

          const appointments = mockAppointmentsStore.get(params.patient_id) || [];
          const newAppointment: MedicalAppointment = {
            id: Date.now(), // Simple ID generation
            user_id: 1,
            ...params,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
          };

          appointments.push(newAppointment);
          mockAppointmentsStore.set(params.patient_id, appointments);

          return newAppointment;
        },
        onSuccess: (_, variables) => {
          queryClient.invalidateQueries({
            queryKey: ["/appointment/by_patient", variables.patient_id],
          });
        },
      });
    },
  },

  update: {
    useMutation: () => {
      return useMutation<
        MedicalAppointment,
        Error,
        { id: number } & UpdateAppointmentParams
      >({
        mutationFn: async ({ id, ...params }) => {
          await new Promise((resolve) => setTimeout(resolve, 300));

          const appointments =
            mockAppointmentsStore.get(params.patient_id) || [];
          const index = appointments.findIndex((a) => a.id === id);

          if (index === -1) throw new Error("Appointment not found");

          const updated = {
            ...appointments[index],
            ...params,
            updated_at: new Date().toISOString(),
          };

          appointments[index] = updated;
          mockAppointmentsStore.set(params.patient_id, appointments);

          return updated;
        },
        onSuccess: (_, variables) => {
          queryClient.invalidateQueries({
            queryKey: ["/appointment/by_patient", variables.patient_id],
          });
        },
      });
    },
  },

  delete: {
    useMutation: () => {
      return useMutation<void, Error, { id: number; patient_id: number }>({
        mutationFn: async ({ id, patient_id }) => {
          await new Promise((resolve) => setTimeout(resolve, 300));

          const appointments = mockAppointmentsStore.get(patient_id) || [];
          const filtered = appointments.filter((a) => a.id !== id);
          mockAppointmentsStore.set(patient_id, filtered);
        },
        onSuccess: (_, variables) => {
          queryClient.invalidateQueries({
            queryKey: ["/appointment/by_patient", variables.patient_id],
          });
        },
      });
    },
  },
};
