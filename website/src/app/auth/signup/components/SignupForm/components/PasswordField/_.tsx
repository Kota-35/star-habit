"use client";

import clsx from "clsx";
import { Eye, EyeOff } from "lucide-react";
import type React from "react";
import {
  type Control,
  Controller,
  type FieldErrors,
  type Path,
} from "react-hook-form";
import type { SignupFormFields } from "@/app/auth/shared/models/signupFormFields";
import { Field, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { userPasswordField } from "./_.hook";

interface Props {
  name: Path<Pick<SignupFormFields, "password" | "confirmPassword">>;
  label: string;
  control: Control<SignupFormFields>;
  errors: FieldErrors<SignupFormFields>;
  placeholder: string;
  onFocus?: React.ComponentProps<"input">["onFocus"];
  onBlur?: React.ComponentProps<"input">["onBlur"];
}

export const PasswordField = (props: Props) => {
  const {
    control,
    errors,
    name,
    label,
    placeholder,
    onTogglePasswordVisibility,
    showPassword,
    onFocus: handleOnFocus,
    onBlur: handleOnBlur,
  } = userPasswordField(props);
  return (
    <Field>
      <FieldLabel htmlFor={name} className="text-black">
        {label}
      </FieldLabel>
      <div className="relative w-full">
        <Controller
          name={name}
          control={control}
          render={({ field }) => (
            <Input
              {...field}
              id={name}
              type={showPassword ? "text" : "password"}
              placeholder={placeholder}
              autoComplete="new-password"
              className={clsx(
                "border-gray-300 py-5 pr-10 text-black placeholder:text-gray-400 focus-visible:border-gray-500 focus-visible:ring-0",
                errors[name] && "border-red-500!",
              )}
              onFocus={handleOnFocus}
              onBlur={handleOnBlur}
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
      {errors[name] && (
        <p className="text-red-500 text-xs">{errors[name].message ?? ""}</p>
      )}
    </Field>
  );
};
