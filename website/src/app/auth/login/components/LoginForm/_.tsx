"use client";

import { FormField } from "@/app/auth/shared/components/FormField";
import { GoogleLogo } from "@/app/auth/shared/components/GoogleIcon";
import { PasswordField } from "@/app/auth/shared/components/PasswordField";

import { Button } from "@/components/ui/button";
import { FieldGroup } from "@/components/ui/field";
import { Separator } from "@/components/ui/separator";
import { useLoginForm } from "./_.hook";

export const LoginForm = () => {
  const { toSignupFormOnClick } = useLoginForm();

  return (
    <div className="flex w-md flex-col rounded-xl bg-white px-6 py-10 shadow-2xl">
      <div className="flex flex-col gap-6">
        <div className="flex flex-col items-center gap-1.5">
          <h1 className="font-bold text-2xl text-black">おかえりなさい</h1>
          <p className="text-black/50">日々の成長を記録しましょう。</p>
        </div>

        <Button
          type="button"
          className="w-full border-black/30 py-5 text-black hover:bg-black/10"
        >
          <GoogleLogo />
          Googleでログイン
        </Button>

        <div className="flex items-center gap-3">
          <Separator className="flex-1 bg-gray-300" />
          <p className="shrink-0 text-black text-xs">
            またはメールアドレスでログイン
          </p>
          <Separator className="flex-1 bg-gray-300" />
        </div>

        <form>
          <FieldGroup>
            <FormField
              id="login-email"
              label="Email"
              type="email"
              placeholder="m@example.com"
            />
            <PasswordField id="login-password" label="パスワード" />
            <Button className="mt-5 rounded-xl bg-blue-600 py-5" type="submit">
              ログイン
            </Button>
          </FieldGroup>
        </form>

        <div className="flex justify-center gap-0.5">
          <div className="text-black/60">アカウントをお持ちでない方は</div>
          <button
            type="button"
            className="text-blue-600 hover:font-semibold hover:text-blue-800 hover:underline"
            onClick={toSignupFormOnClick}
          >
            こちら
          </button>
        </div>
      </div>
    </div>
  );
};
