import { zodResolver } from "@hookform/resolvers/zod";
import { Key } from "lucide-react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import z from "zod";
import { APIHooks } from "@/api/hooks";
import { FormProvider } from "@/components/form/FormProvider";
import { Button, Label } from "@/components/ui";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { login } from "@/lib/authUtils";
import { formatAccessKey } from "@/lib/utils";

interface AccessKeyModalProps {
  userEmail: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export const AccessKeyModal = ({
  userEmail,
  open,
  onOpenChange,
}: AccessKeyModalProps) => {
  const { t } = useTranslation();

  const checkAccessKeyMutation = APIHooks.auth.checkAccessKey.useMutation();

  const checkAccessKeySchema = z.object({
    accessKey: z
      .string()
      .trim()
      .min(1, t("auth.accessKey.validation.accessKeyRequired")),
  });

  const checkAccessKeyForm = useForm({
    resolver: zodResolver(checkAccessKeySchema),
    defaultValues: {
      accessKey: "",
    },
  });

  const onSubmit = checkAccessKeyForm.handleSubmit(async (data) => {
    checkAccessKeyMutation.mutateAsync(
      {
        user_email: userEmail,
        access_key: data.accessKey,
      },
      {
        onSuccess: (res) => {
          login(res.token);
        },
        onError: (error) => {
          checkAccessKeyForm.setError("accessKey", {
            message: error.response?.data.msg,
          });
        },
      },
    );
  });

  console.log(checkAccessKeyForm.watch("accessKey"));

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md bg-white dark:bg-gray-900 shadow-lg border-0 backdrop-blur-sm">
        <DialogHeader className="space-y-2 text-center pb-4">
          <DialogTitle className="text-2xl font-bold tracking-tight text-foreground">
            {t("auth.accessKey.title")}
          </DialogTitle>
          <DialogDescription className="text-muted-foreground">
            {t("auth.accessKey.description")}
          </DialogDescription>
        </DialogHeader>

        <FormProvider
          methods={checkAccessKeyForm}
          onSubmit={onSubmit}
          className="space-y-4"
        >
          <div className="space-y-2">
            <Label htmlFor="accessKey" className="text-sm font-medium">
              {t("auth.accessKey.accessKey")}
            </Label>
            <div className="relative">
              <Key className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                type="text"
                placeholder={t("auth.accessKey.accessKeyPlaceholder")}
                className="pl-10 h-11"
                value={formatAccessKey(
                  checkAccessKeyForm.watch("accessKey") || "",
                )}
                onChange={(e) => {
                  const formatted = formatAccessKey(e.target.value);
                  checkAccessKeyForm.setValue("accessKey", formatted);
                }}
              />
            </div>
            {checkAccessKeyForm.formState.errors.accessKey && (
              <p className="text-sm font-medium text-destructive">
                {checkAccessKeyForm.formState.errors.accessKey.message}
              </p>
            )}
          </div>

          <div className="flex gap-3 pt-4">
            <Button
              type="button"
              variant="outline"
              className="flex-1 h-11"
              onClick={() => onOpenChange(false)}
            >
              {t("auth.accessKey.cancel")}
            </Button>
            <Button
              type="submit"
              className="flex-1 h-11 text-sm font-medium"
              disabled={checkAccessKeyForm.formState.isSubmitting}
            >
              {checkAccessKeyForm.formState.isSubmitting ? (
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin" />
                  {t("auth.accessKey.verifying")}
                </div>
              ) : (
                t("auth.accessKey.verify")
              )}
            </Button>
          </div>
        </FormProvider>
      </DialogContent>
    </Dialog>
  );
};
