import { useFormContext } from "react-hook-form";
import { DatePicker } from "../date-picker";
import { FormControl, FormField, FormItem, FormMessage } from "../ui/form";

type Props = {
  name: string;
  label: string;
  disabled?: boolean;
};

export const FormDatePicker: React.FC<Props> = ({ name, label, disabled }) => {
  const form = useFormContext();

  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <DatePicker
              label={label}
              value={field.value}
              onChange={field.onChange}
              disabled={disabled}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  );
};
