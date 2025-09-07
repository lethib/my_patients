import { createFileRoute } from "@tanstack/react-router";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { H2 } from "@/components/ui/typography/h2";
import { AddPatientModal } from "./components/AddPatientModal";

export const Route = createFileRoute("/search/")({
  component: Search,
});

function Search() {
  const [isAddPatientModalOpened, setIsAddPatientModalOpened] = useState(false);
  return (
    <>
      <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-background via-background to-muted/20 px-4">
        <H2>Find a patient</H2>
        <Button onClick={() => setIsAddPatientModalOpened(true)}>
          Add a patient
        </Button>
      </div>

      <AddPatientModal
        open={isAddPatientModalOpened}
        setIsOpen={() => setIsAddPatientModalOpened(false)}
      />
    </>
  );
}
