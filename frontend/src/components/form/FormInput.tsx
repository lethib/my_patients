import { useFormContext } from "react-hook-form";
import { FormControl, FormField, FormItem, FormMessage } from "../ui/form.tsx";
import { Input } from "../ui/input.tsx";

type Props = {
  name: string;
  icon?: React.ReactNode;
} & React.InputHTMLAttributes<HTMLInputElement>;

export const FormInput: React.FC<Props> = ({
  name,
  className,
  icon,
  ...inputProps
}) => {
  const form = useFormContext();

  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <div className="relative">
              {icon}
              <Input {...field} {...inputProps} className={className} />
            </div>
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  );
};
