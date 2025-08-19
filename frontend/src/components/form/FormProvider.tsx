import type { FC } from "react";
import type { UseFormReturn } from "react-hook-form";
import { Form } from "../ui/form";

type Props = {
  children: React.ReactNode;
  // biome-ignore lint/suspicious/noExplicitAny: Using any for simplicity
  methods: UseFormReturn<any>;
  onSubmit?: () => void;
  className?: string;
};

export const FormProvider: FC<Props> = ({
  children,
  onSubmit,
  methods,
  className,
}) => (
  <Form {...methods}>
    <form onSubmit={onSubmit} className={className}>
      {children}
    </form>
  </Form>
);
