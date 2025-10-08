import { ChevronDownIcon } from "lucide-react";
import * as React from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import { Label } from "@/components/ui/label";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

type Props = {
  label: string;
  value?: Date;
  onChange: (date: Date | undefined) => void;
  disabled?: boolean;
};

export function DatePicker({ label, value, onChange, disabled }: Props) {
  const now = new Date();

  const { t } = useTranslation();
  const [open, setOpen] = React.useState(false);

  const startMonth = new Date(now.getFullYear() - 1, now.getMonth());
  const endMonth = new Date(now.getFullYear(), now.getMonth());

  return (
    <div className="flex flex-col gap-3">
      <Label htmlFor="date" className="px-1">
        {label}
      </Label>
      <Popover open={open} onOpenChange={setOpen} modal>
        <PopoverTrigger asChild>
          <Button
            variant="outline"
            id="date"
            className="w-full justify-between font-normal"
            disabled={disabled}
          >
            {value
              ? value.toLocaleDateString()
              : t("components.datePicker.selectDate")}
            <ChevronDownIcon />
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-auto overflow-hidden p-0" align="start">
          <Calendar
            mode="single"
            selected={value}
            captionLayout="dropdown"
            startMonth={startMonth}
            endMonth={endMonth}
            disabled={{ after: now }}
            onSelect={(date) => {
              onChange(date);
              setOpen(false);
            }}
          />
        </PopoverContent>
      </Popover>
    </div>
  );
}
