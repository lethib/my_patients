import type React from "react";

export const H2 = ({ children, ...props }: React.ComponentProps<"h2">) => (
  <h2
    className="scroll-m-20 pb-2 text-3xl font-semibold tracking-tight first:mt-0"
    {...props}
  >
    {children}
  </h2>
);
