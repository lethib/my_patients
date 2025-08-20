import { zodResolver } from "@hookform/resolvers/zod";
import { Eye, EyeOff, Lock, Mail, User } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import z from "zod";
import { Button, Label } from "@/components/ui";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { APIClient } from "./api/api";
import { FormInput } from "./components/form/FormInput";
import { FormProvider } from "./components/form/FormProvider";

interface RegisterModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function RegisterModal({ open, onOpenChange }: RegisterModalProps) {
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);

  const registerMutation = APIClient.hooks.auth.register.useMutation();

  const registerFormSchema = z
    .object({
      firstName: z.string().trim().nonempty("First name is required"),
      lastName: z.string().trim().nonempty("Last name is required"),
      email: z.email("Invalid email address"),
      password: z.string().min(6, "Password must be at least 6 characters"),
      confirmPassword: z.string(),
    })
    .refine((data) => data.password === data.confirmPassword, {
      error: "Passwords don't match",
      path: ["confirmPassword"],
    });

  const registerForm = useForm({
    resolver: zodResolver(registerFormSchema),
    defaultValues: {
      firstName: "",
      lastName: "",
      email: "",
      password: "",
      confirmPassword: "",
    },
  });

  const onSubmit = registerForm.handleSubmit(async (data) => {
    registerMutation.mutateAsync(
      {
        email: data.email,
        password: data.password,
        name: `${data.firstName} ${data.lastName}`,
      },
      {
        onSuccess: () => {
          alert(
            "Registration successful! Please check your email to verify your account.",
          );
          onOpenChange(false);
          registerForm.reset();
        },
        onError: (error) => {
          alert(`Registration failed: ${error.message}`);
        },
      },
    );
  });

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md bg-white dark:bg-gray-900 shadow-lg border-0 backdrop-blur-sm">
        <DialogHeader className="space-y-2 text-center pb-4">
          <DialogTitle className="text-2xl font-bold tracking-tight text-foreground">
            Create Account
          </DialogTitle>
          <DialogDescription className="text-muted-foreground">
            Fill in your information to get started
          </DialogDescription>
        </DialogHeader>

        <FormProvider
          methods={registerForm}
          onSubmit={onSubmit}
          className="space-y-4"
        >
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="firstName" className="text-sm font-medium">
                First Name
              </Label>
              <FormInput
                id="firstName"
                name="firstName"
                type="text"
                placeholder="First name"
                className="pl-10 h-11"
                icon={
                  <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="lastName" className="text-sm font-medium">
                Last Name
              </Label>
              <FormInput
                id="lastName"
                name="lastName"
                type="text"
                placeholder="Last name"
                className="pl-10 h-11"
                icon={
                  <User className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="email" className="text-sm font-medium">
              Email
            </Label>
            <FormInput
              id="email"
              name="email"
              type="email"
              placeholder="Enter your email"
              className="pl-10 h-11"
              icon={
                <Mail className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              }
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="password" className="text-sm font-medium">
              Password
            </Label>
            <div className="relative">
              <FormInput
                id="password"
                name="password"
                type={showPassword ? "text" : "password"}
                placeholder="Create a password"
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

          <div className="space-y-2">
            <Label htmlFor="confirmPassword" className="text-sm font-medium">
              Confirm Password
            </Label>
            <div className="relative">
              <FormInput
                id="confirmPassword"
                name="confirmPassword"
                type={showConfirmPassword ? "text" : "password"}
                placeholder="Confirm your password"
                className="pl-10 pr-10 h-11"
                icon={
                  <Lock className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                }
              />
              <button
                type="button"
                onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
              >
                {showConfirmPassword ? (
                  <EyeOff className="h-4 w-4" />
                ) : (
                  <Eye className="h-4 w-4" />
                )}
              </button>
            </div>
          </div>

          <div className="flex gap-3 pt-4">
            <Button
              type="button"
              variant="outline"
              className="flex-1 h-11"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              className="flex-1 h-11 text-sm font-medium"
              disabled={registerForm.formState.isLoading}
            >
              {registerForm.formState.isLoading ? (
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin" />
                  Creating...
                </div>
              ) : (
                "Create Account"
              )}
            </Button>
          </div>
        </FormProvider>

        <div className="text-center pt-4 border-t">
          <p className="text-sm text-muted-foreground">
            Already have an account?{" "}
            <button
              className="text-primary hover:underline font-medium"
              onClick={() => onOpenChange(false)}
            >
              Sign in instead
            </button>
          </p>
        </div>
      </DialogContent>
    </Dialog>
  );
}
