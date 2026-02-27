import type React from "react";
import { Field, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

interface Props {
  id: string;
  type: React.ComponentProps<"input">["type"];
  label: string;
  placeholder: string;
}

export const FormField = (props: Props) => {
  const { id, type, label, placeholder } = props;
  return (
    <Field>
      <FieldLabel htmlFor={id} className="text-black">
        {label}
      </FieldLabel>
      <Input
        id={id}
        type={type}
        placeholder={placeholder}
        required
        className="border-gray-300 py-5 text-black placeholder:text-gray-400 focus-visible:border-gray-500 focus-visible:ring-0"
      />
    </Field>
  );
};
