import { useFormContext } from "react-hook-form";
import { Label } from "../ui";
import { FormControl, FormField, FormItem, FormMessage } from "../ui/form";
import { Switch, type SwitchProps } from "../ui/switch";

type Props = {
  name: string;
  label: string;
} & Omit<SwitchProps, "checked" | "onCheckedChange">;

export const FormSwitch = ({ name, label, ...switchProps }: Props) => {
  const form = useFormContext();

  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <div className="flex items-center space-x-3">
              <Switch
                checked={field.value}
                onCheckedChange={field.onChange}
                {...switchProps}
              />
              <Label htmlFor={name} className="cursor-pointer">
                {label}
              </Label>
            </div>
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  );
};
