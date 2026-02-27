"use client";

import { Eye, EyeOff } from "lucide-react";
import { Field, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { usePasswordField } from "./_.hook";

interface Props {
  id: string;
  label: string;
}

export const PasswordField = (props: Props) => {
  const { id, label, visible, setVisible } = usePasswordField(props);

  return (
    <Field>
      <FieldLabel htmlFor={id} className="text-black">
        {label}
      </FieldLabel>
      <div className="relative w-full">
        <Input
          id={id}
          type={visible ? "text" : "password"}
          required
          placeholder="8文字以上の英角英数字"
          className="border-gray-300 py-5 pr-10 text-black placeholder:text-gray-400 focus-visible:border-gray-500 focus-visible:ring-0"
          autoComplete="new-password"
        />
        <button
          type="button"
          aria-label={visible ? "パスワードを隠す" : "パスワードを表示"}
          onClick={() => setVisible((v) => !v)}
          className="-translate-y-1/2 absolute top-1/2 right-2 rounded p-1 text-gray-500 hover:text-gray-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-gray-400 focus-visible:ring-offset-0"
        >
          {visible ? (
            <EyeOff className="h-4 w-4" aria-hidden />
          ) : (
            <Eye className="h-4 w-4" aria-hidden />
          )}
        </button>
      </div>
    </Field>
  );
};
