"use client";

import { Loader2 } from "lucide-react";

import { GoogleLogo } from "@/app/auth/shared/components/GoogleIcon";
import { Button } from "@/components/ui/button";
import { FieldGroup } from "@/components/ui/field";
import { Separator } from "@/components/ui/separator";
import { useSignupForm } from "./_.hook";
import { FormField } from "./components/FormField";
import { PasswordField } from "./components/PasswordField";
import { PasswordStrength } from "./components/PasswordStrength";

export const SignupForm = () => {
  const {
    toLoginFormOnClick,
    signupFormOnSubmit,
    isSubmitPending,
    register,
    control,
    errors,
    password,
    passwordFocus,
    passwordFieldOnBlur,
    passwordFieldOnFocus,
  } = useSignupForm();

  return (
    <div className="flex h-full items-center justify-center">
      <div className="flex w-md flex-col gap-4">
        <div>
          <h1 className="mb-3 font-bold text-3xl text-black">
            アカウントを作成
          </h1>
          <div className="mb-5 flex gap-1 font-semibold">
            <p className="text-black/60">すでにアカウントをお持ちですか？</p>
            <button
              className="text-blue-600 hover:font-semibold hover:text-blue-800 hover:underline"
              type="button"
              onClick={toLoginFormOnClick}
            >
              ログインはこちら
            </button>
          </div>
        </div>

        <Button
          type="button"
          className="w-full border-black/30 py-5 text-black hover:bg-black/10"
        >
          <GoogleLogo />
          Googleで登録
        </Button>

        <div className="flex items-center gap-3">
          <Separator className="flex-1 bg-gray-300" />
          <p className="shrink-0 text-black text-xs">
            またはメールアドレスで登録
          </p>
          <Separator className="flex-1 bg-gray-300" />
        </div>

        <form onSubmit={signupFormOnSubmit}>
          <FieldGroup>
            {errors.root?.message && (
              <p className="text-red-500 text-xs">{errors.root.message}</p>
            )}
            <FormField
              name="username"
              label="名前"
              register={register}
              errors={errors}
              placeholder="山田 太郎"
            />

            <FormField
              name="email"
              label="メールアドレス"
              register={register}
              errors={errors}
              placeholder="m@example.com"
            />

            <div className="space-y-1">
              <PasswordField
                name="password"
                label="パスワード"
                control={control}
                errors={errors}
                placeholder=""
                onFocus={passwordFieldOnFocus}
                onBlur={passwordFieldOnBlur}
              />

              <PasswordStrength
                password={password}
                isVisible={!passwordFocus && password.length > 0}
              />
            </div>

            <PasswordField
              name="confirmPassword"
              label="パスワード(確認)"
              control={control}
              errors={errors}
              placeholder=""
            />

            <Button
              className="mt-5 rounded-xl bg-blue-600 py-5 text-white"
              type="submit"
              disabled={isSubmitPending}
            >
              {isSubmitPending ? (
                <>
                  <Loader2 className="size-5 animate-spin" aria-hidden />
                  送信中
                </>
              ) : (
                "アカウントを作成する"
              )}
            </Button>
          </FieldGroup>
        </form>
      </div>
    </div>
  );
};
