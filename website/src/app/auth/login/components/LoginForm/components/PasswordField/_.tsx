"use client";

import clsx from "clsx";
import { Eye, EyeOff } from "lucide-react";
import { type Control, Controller, type FieldErrors } from "react-hook-form";
import type { LoginFormFields } from "@/app/auth/shared/models/loginFormFields";
import { Field, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { userPasswordField } from "./_.hook";

interface Props {
  control: Control<LoginFormFields>;
  errors: FieldErrors<LoginFormFields>;
}

export const PasswordField = (props: Props) => {
  const { control, errors, onTogglePasswordVisibility, showPassword } =
    userPasswordField(props);
  return (
    <Field>
      <FieldLabel htmlFor="password" className="text-black">
        パスワード
      </FieldLabel>
      <div className="relative w-full">
        <Controller
          name="password"
          control={control}
          render={({ field }) => (
            <Input
              {...field}
              id="password"
              type={showPassword ? "text" : "password"}
              autoComplete="new-password"
              className={clsx(
                "border-gray-300 py-5 pr-10 text-black placeholder:text-gray-400 focus-visible:border-gray-500 focus-visible:ring-0",
                errors.password && "border-red-500!",
              )}
            />
          )}
        />
        <button
          type="button"
          onClick={onTogglePasswordVisibility}
          className="-translate-y-1/2 absolute top-1/2 right-3 text-gray-500 hover:text-gray-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-gray-400 focus-visible:ring-offset-0"
          aria-label={showPassword ? "パスワードを隠す" : "パスワードを表示"}
        >
          {showPassword ? (
            <EyeOff className="h-4 w-4" aria-hidden />
          ) : (
            <Eye className="h-4 w-4" aria-hidden />
          )}
        </button>
      </div>
      {errors.password && (
        <p className="text-red-500 text-xs">{errors.password.message ?? ""}</p>
      )}
    </Field>
  );
};
