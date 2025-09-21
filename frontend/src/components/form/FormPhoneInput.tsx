import { forwardRef } from "react";
import { useFormContext } from "react-hook-form";
import { useTranslation } from "react-i18next";
import PhoneInput, { type Country } from "react-phone-number-input";
import { cn } from "@/lib/utils";
import "react-phone-number-input/style.css";
import { FormControl, FormField, FormItem, FormMessage } from "../ui/form.tsx";

interface FormPhoneInputProps {
  name: string;
  placeholder?: string;
  defaultCountry?: Country;
  required?: boolean;
  disabled?: boolean;
  className?: string;
}

const PhoneInputComponent = forwardRef<
  HTMLInputElement,
  React.InputHTMLAttributes<HTMLInputElement> & { placeholder?: string }
>(({ className, placeholder, ...props }, ref) => {
  return (
    <input
      {...props}
      ref={ref}
      placeholder={placeholder}
      className={cn(
        "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-11 w-full min-w-0 rounded-md border bg-transparent pl-3 pr-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-ring focus-visible:ring-ring/80 focus-visible:ring-[2px]",
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        className,
      )}
    />
  );
});

PhoneInputComponent.displayName = "PhoneInputComponent";

export const FormPhoneInput: React.FC<FormPhoneInputProps> = ({
  name,
  placeholder,
  defaultCountry = "FR",
  required = false,
  disabled = false,
  className,
}) => {
  const form = useFormContext();
  const { t } = useTranslation();

  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <div className={cn("relative", className)}>
              <PhoneInput
                international
                countryCallingCodeEditable={false}
                defaultCountry={defaultCountry}
                value={field.value}
                onChange={field.onChange}
                disabled={disabled}
                required={required}
                className="phone-input-custom"
                inputComponent={PhoneInputComponent}
                placeholder={
                  placeholder || t("auth.register.phoneNumberPlaceholder")
                }
              />
              <style>{`
                .phone-input-custom {
                  position: relative;
                }

                .phone-input-custom .PhoneInputCountrySelect {
                  position: absolute;
                  left: 12px;
                  top: 50%;
                  transform: translateY(-50%);
                  background: transparent;
                  border: none;
                  outline: none;
                  z-index: 10;
                  font-size: 14px;
                  color: inherit;
                  padding: 0;
                  margin: 0;
                  cursor: pointer;
                  height: 20px;
                  display: flex;
                  align-items: center;
                }

                .phone-input-custom .PhoneInputCountrySelect:focus {
                  outline: none;
                }

                .phone-input-custom .PhoneInputCountrySelectArrow {
                  display: none;
                }

                .phone-input-custom .PhoneInputCountryIcon {
                  width: 20px;
                  height: 15px;
                  margin-right: 6px;
                  border-radius: 2px;
                  object-fit: cover;
                }

                @media (max-width: 768px) {
                  .phone-input-custom .PhoneInputCountrySelect {
                    font-size: 14px;
                  }

                  .phone-input-custom .PhoneInputInput {
                    padding-left: 55px !important;
                  }
                }
              `}</style>
            </div>
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  );
};
