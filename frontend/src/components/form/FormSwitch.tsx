import { useFormContext } from "react-hook-form";
import { FormControl, FormField, FormItem, FormMessage } from "../ui/form";
import { Switch, type SwitchProps } from "../ui/switch";

type Props = {
  name: string;
} & Omit<SwitchProps, "checked" | "onCheckedChange">;

export const FormSwitch = ({ name, ...switchProps }: Props) => {
  const form = useFormContext();

  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <Switch
              checked={field.value}
              onCheckedChange={field.onChange}
              {...switchProps}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  );
};
