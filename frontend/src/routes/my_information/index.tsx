import { createFileRoute } from "@tanstack/react-router";
import { BusinessInformationCard } from "./components/BusinessInformationCard";
import { SignatureCard } from "./components/SignatureCard";

export const Route = createFileRoute("/my_information/")({
  component: MyInformation,
});

function MyInformation() {
  return (
    <div className="container mx-auto p-6 max-w-2xl">
      <BusinessInformationCard />
      <SignatureCard />
    </div>
  );
}
