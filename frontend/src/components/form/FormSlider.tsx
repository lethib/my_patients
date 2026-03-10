import { useFormContext } from "react-hook-form";
import { FormControl, FormField, FormItem, FormMessage } from "../ui/form";
import { Slider, type SliderProps } from "../ui/slider";

type Props = {
  name: string;
} & Omit<SliderProps, "value" | "onValueChange">;

export const FormSlider = ({ name, ...sliderProps }: Props) => {
  const form = useFormContext();

  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <Slider
              value={[field.value ?? 0]}
              onValueChange={(val) => field.onChange(val[0])}
              {...sliderProps}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  );
};
