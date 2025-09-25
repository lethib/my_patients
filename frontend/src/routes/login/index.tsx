import { zodResolver } from "@hookform/resolvers/zod";
import { createFileRoute, Navigate, useNavigate } from "@tanstack/react-router";
import { Eye, EyeOff, Lock, Mail } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import * as z from "zod";
import { APIClient } from "@/api/api";
import { FormInput } from "@/components/form/FormInput";
import { FormProvider } from "@/components/form/FormProvider";
import { Button, Label } from "@/components/ui";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useCurrentUser } from "@/hooks/useCurrentUser";
import { RegisterModal } from "./components/RegisterModal";

export const Route = createFileRoute("/login/")({
  component: Login,
});

function Login() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { currentUser } = useCurrentUser();
  const [showPassword, setShowPassword] = useState(false);
  const [isRegisterModalOpen, setIsRegisterModalOpen] = useState(false);

  const loginMutation = APIClient.hooks.auth.login.useMutation();

  const loginFormSchema = z.object({
    email: z.string().email(t("auth.login.validation.invalidEmail")),
    password: z.string().min(1, t("auth.login.validation.passwordRequired")),
  });

  const loginForm = useForm({
    resolver: zodResolver(loginFormSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const onSubmit = async (data: z.infer<typeof loginFormSchema>) => {
    loginMutation.mutateAsync(data, {
      onSuccess: (res) => {
        localStorage.setItem("accessToken", res.token);
        navigate({ to: "/", replace: true });
      },
      onError: (error) => {
        alert(`${t("auth.login.error")}: ${error.message}`);
      },
    });
  };

  if (currentUser) return <Navigate to="/" />;

  return (
    <>
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-background via-background to-muted/20 px-4">
        <Card className="w-full max-w-md shadow-lg border-0 bg-card/50 backdrop-blur-sm">
          <CardHeader className="flex flex-col items-center space-y-2 pb-6">
            <img
              src="/favicon/favicon.svg"
              height={100}
              width={100}
              className="-mb-1"
            />
            <CardTitle className="text-3xl font-bold tracking-tight">
              {t("auth.login.title")}
            </CardTitle>
            <CardDescription className="text-muted-foreground">
              {t("auth.login.description")}
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-6">
            <FormProvider
              methods={loginForm}
              onSubmit={loginForm.handleSubmit((data) => onSubmit(data))}
              className="space-y-4"
            >
              <div className="space-y-2">
                <Label htmlFor="email" className="text-sm font-medium">
                  {t("auth.login.email")}
                </Label>
                <FormInput
                  name="email"
                  id="email"
                  type="email"
                  placeholder={t("auth.login.emailPlaceholder")}
                  className="pl-10 h-11"
                  icon={
                    <Mail className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                  }
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="password" className="text-sm font-medium">
                  {t("auth.login.password")}
                </Label>
                <div className="relative">
                  <FormInput
                    name="password"
                    id="password"
                    type={showPassword ? "text" : "password"}
                    placeholder={t("auth.login.passwordPlaceholder")}
                    className="pl-10 pr-10 h-11"
                    icon={
                      <Lock className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                    }
                  />
                  <button
                    type="button"
                    onClick={() => setShowPassword(!showPassword)}
                    className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                  >
                    {showPassword ? (
                      <EyeOff className="h-4 w-4" />
                    ) : (
                      <Eye className="h-4 w-4" />
                    )}
                  </button>
                </div>
              </div>

              <Button
                type="submit"
                className="w-full h-11 text-sm font-medium"
                disabled={loginMutation.isPending}
              >
                {loginMutation.isPending ? (
                  <div className="flex items-center gap-2">
                    <div className="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin" />
                    {t("auth.login.signingIn")}
                  </div>
                ) : (
                  t("auth.login.signIn")
                )}
              </Button>
            </FormProvider>

            <div className="text-center space-y-2">
              {/* <button className="text-sm text-primary hover:underline">
                {t('auth.login.forgotPassword')}
              </button> */}
              <p className="text-sm text-muted-foreground">
                {t("auth.login.noAccount")}{" "}
                <button
                  className="text-primary hover:underline font-medium"
                  onClick={() => setIsRegisterModalOpen(true)}
                >
                  {t("auth.login.signUp")}
                </button>
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      <RegisterModal
        open={isRegisterModalOpen}
        onOpenChange={setIsRegisterModalOpen}
      />
    </>
  );
}
