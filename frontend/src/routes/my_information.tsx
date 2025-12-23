import { createFileRoute } from "@tanstack/react-router";
import { BusinessInformationCard } from "./my_information/components/BusinessInformationCard";
import { SignatureModal } from "./my_information/components/SignatureCard";

export const Route = createFileRoute("/my_information")({
  component: MyInformation,
});

function MyInformation() {
  return (
    <div className="container mx-auto p-6 max-w-2xl">
      <BusinessInformationCard />
      <SignatureModal />
    </div>
  );
}
