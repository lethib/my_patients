import { LoaderCircleIcon, type LucideProps } from "lucide-react";
import { cn } from "@/lib/utils";

export type SpinnerProps = Omit<LucideProps, "variant">;

const Default = ({ className, ...props }: SpinnerProps) => (
  <LoaderCircleIcon className={cn("animate-spin", className)} {...props} />
);

export const Spinner = ({ ...props }: SpinnerProps) => {
  return <Default {...props} />;
};

export const CenteredSpineer = ({ ...props }: SpinnerProps) => (
  <div className="flex justify-center items-center">
    <Spinner {...props} />
  </div>
);
