import { zodResolver } from "@hookform/resolvers/zod";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Building2, FileText, PenTool, Upload, Users } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import z from "zod";
import { queryClient } from "@/api/api";
import { APIHooks } from "@/api/hooks";
import { FormInput } from "@/components/form/FormInput";
import { FormProvider } from "@/components/form/FormProvider";
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Label,
} from "@/components/ui";
import { useCurrentUser } from "@/hooks/useCurrentUser";

export const Route = createFileRoute("/my_information")({
  component: MyInformation,
});

const businessInfoSchema = z.object({
  rpps_number: z.string().trim().length(11),
  siret_number: z.string().trim().length(14),
  adeli_number: z.string().trim().optional(),
});

function MyInformation() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { currentUser } = useCurrentUser();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [uploadStatus, setUploadStatus] = useState<"idle" | "success" | "error">("idle");

  const saveBusinessInformationMutation =
    APIHooks.user.saveBusinessInformation.useMutation();
  const uploadSignatureMutation = APIHooks.user.uploadSignature.useMutation();

  const businessForm = useForm({
    resolver: zodResolver(businessInfoSchema),
    defaultValues: {
      rpps_number: "",
      siret_number: "",
      adeli_number: "",
    },
  });

  useEffect(() => {
    if (currentUser?.business_information) {
      businessForm.reset({
        rpps_number: currentUser.business_information.rpps_number || "",
        siret_number: currentUser.business_information.siret_number || "",
        adeli_number: currentUser.business_information.adeli_number || "",
      });
    }
  }, [currentUser, businessForm]);

  const onSubmit = businessForm.handleSubmit(async (values) => {
    saveBusinessInformationMutation.mutateAsync(values).then(() => {
      queryClient.invalidateQueries({ queryKey: ["/auth/me"] });
      alert(t("businessInfo.successMessage"));
      navigate({ to: "/search" });
    });
  });

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      // Validate file type
      const validTypes = ["image/png", "image/jpeg", "image/jpg"];
      if (!validTypes.includes(file.type)) {
        alert(t("signature.invalidFileType"));
        return;
      }

      // Validate file size (max 200KB)
      const maxSize = 200 * 1024;
      if (file.size > maxSize) {
        alert(t("signature.fileTooLarge"));
        return;
      }

      setSelectedFile(file);
      setUploadStatus("idle");
    }
  };

  const handleUploadSignature = async () => {
    if (!selectedFile) return;

    try {
      await uploadSignatureMutation.mutateAsync(selectedFile);
      setUploadStatus("success");
      setSelectedFile(null);
      if (fileInputRef.current) {
        fileInputRef.current.value = "";
      }
    } catch (error) {
      setUploadStatus("error");
      console.error("Upload error:", error);
    }
  };

  return (
    <div className="container mx-auto p-6 max-w-2xl">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Building2 className="h-5 w-5" />
            {t("businessInfo.title")}
          </CardTitle>
          <CardDescription>{t("businessInfo.subtitle")}</CardDescription>
        </CardHeader>
        <CardContent>
          <FormProvider
            methods={businessForm}
            onSubmit={onSubmit}
            className="space-y-6"
          >
            <div className="space-y-2">
              <Label htmlFor="rpps_number" className="text-sm font-medium">
                {t("businessInfo.rppsNumber")} *
              </Label>
              <FormInput
                id="rpps_number"
                name="rpps_number"
                type="text"
                placeholder={t("businessInfo.rppsPlaceholder")}
                className="pl-10 h-11"
                icon={
                  <Users className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="siret_number" className="text-sm font-medium">
                {t("businessInfo.siretNumber")} *
              </Label>
              <FormInput
                id="siret_number"
                name="siret_number"
                type="text"
                placeholder={t("businessInfo.siretPlaceholder")}
                className="pl-10 h-11"
                icon={
                  <Building2 className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="adeli_number" className="text-sm font-medium">
                {t("businessInfo.adeliNumber")}
              </Label>
              <FormInput
                id="adeli_number"
                name="adeli_number"
                type="text"
                placeholder={t("businessInfo.adeliPlaceholder")}
                className="pl-10 h-11"
                icon={
                  <FileText className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>

            <Button type="submit" className="w-full">
              {t("businessInfo.save")}
            </Button>
          </FormProvider>
        </CardContent>
      </Card>

      <Card className="mt-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <PenTool className="h-5 w-5" />
            {t("signature.title")}
          </CardTitle>
          <CardDescription>{t("signature.subtitle")}</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="signature" className="text-sm font-medium">
              {t("signature.selectFile")}
            </Label>
            <input
              ref={fileInputRef}
              id="signature"
              type="file"
              accept="image/png,image/jpeg,image/jpg"
              onChange={handleFileSelect}
              className="hidden"
            />
            <div className="flex items-center gap-3">
              <Button
                type="button"
                variant="outline"
                onClick={() => fileInputRef.current?.click()}
                className="flex-1"
              >
                <Upload className="mr-2 h-4 w-4" />
                {selectedFile ? selectedFile.name : t("signature.chooseFile")}
              </Button>
              {selectedFile && (
                <Button
                  type="button"
                  onClick={handleUploadSignature}
                  disabled={uploadSignatureMutation.isPending}
                  className="px-8"
                >
                  {uploadSignatureMutation.isPending
                    ? t("signature.uploading")
                    : t("signature.upload")}
                </Button>
              )}
            </div>
            <p className="text-xs text-muted-foreground">
              {t("signature.fileRequirements")}
            </p>
          </div>

          {uploadStatus === "success" && (
            <div className="rounded-md bg-green-50 p-3 text-sm text-green-800">
              {t("signature.uploadSuccess")}
            </div>
          )}

          {uploadStatus === "error" && (
            <div className="rounded-md bg-red-50 p-3 text-sm text-red-800">
              {t("signature.uploadError")}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
