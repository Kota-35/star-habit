import { Button } from "@/components/ui/button";
import { FieldGroup } from "@/components/ui/field";
import { Separator } from "@/components/ui/separator";
import { FormField } from "./components/FormField";
import { PasswordField } from "./components/PasswordField";

export const SignupForm = () => {
  return (
    <div className="flex h-full items-center justify-center">
      <div className="flex w-md flex-col gap-4">
        <div>
          <h1 className="mb-3 font-bold text-3xl text-black">
            アカウントを作成
          </h1>
          <div className="mb-5 flex gap-1 font-semibold">
            <p className="text-black/60">すでにアカウントをお持ちですか？</p>
            <div className="text-blue-600 hover:text-blue-800 hover:underline">
              <p>ログインはこちら</p>
            </div>
          </div>
        </div>

        <Button
          type="button"
          className="w-full border-black/30 py-5 text-black hover:bg-black/10"
        >
          <svg
            aria-label="Google"
            className="mr-2 h-4 w-4"
            role="img"
            viewBox="0 0 24 24"
          >
            <path
              d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
              fill="#4285F4"
            />
            <path
              d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
              fill="#34A853"
            />
            <path
              d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
              fill="#FBBC05"
            />
            <path
              d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
              fill="#EA4335"
            />
          </svg>
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
