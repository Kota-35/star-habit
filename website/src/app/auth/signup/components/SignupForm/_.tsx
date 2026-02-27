"use client";

import { GoogleLogo } from "@/app/auth/shared/components/GoogleIcon";
import { Button } from "@/components/ui/button";
import { FieldGroup } from "@/components/ui/field";
import { Separator } from "@/components/ui/separator";
import { FormField } from "../../../shared/components/FormField";
import { PasswordField } from "../../../shared/components/PasswordField";
import { useSignupForm } from "./_.hook";

export const SignupForm = () => {
  const { toLoginFormOnClick } = useSignupForm();

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

        <form>
          <FieldGroup>
            <FormField
              id="name"
              label="名前"
              type="text"
              placeholder="山田 太郎"
            />

            <FormField
              id="email"
              label="Email"
              type="email"
              placeholder="m@example.com"
            />

            <PasswordField id="password" label="パスワード" />

            <Button className="mt-5 rounded-xl bg-blue-600 py-5" type="submit">
              アカウントを作成する
            </Button>
          </FieldGroup>
        </form>
      </div>
    </div>
  );
};
