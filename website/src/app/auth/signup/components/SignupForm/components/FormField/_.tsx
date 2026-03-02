import type { FieldErrors, Path, UseFormRegister } from "react-hook-form";
import { match } from "ts-pattern";
import type { SignupFormFields } from "@/app/auth/shared/models/signupFormFields";
import { Field, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

interface Props {
  name: Path<Pick<SignupFormFields, "username" | "email">>;
  label: string;
  register: UseFormRegister<SignupFormFields>;
  errors: FieldErrors<SignupFormFields>;
  placeholder: string;
}

export const FormField = (props: Props) => {
  const { name, register, errors, placeholder, label } = props;
  return (
    <Field>
      <FieldLabel htmlFor={name} className="text-black">
        {label}
      </FieldLabel>
      <Input
        id={name}
        type={match(name)
          .with("username", () => "text")
          .with("email", () => "email")
          .exhaustive()}
        placeholder={placeholder}
        {...register(name)}
        className="border-gray-300 py-5 text-black placeholder:text-gray-400 focus-visible:border-gray-500 focus-visible:ring-0"
      />
      {errors.email && (
        <p className="text-red-500 text-xs">{errors.email.message}</p>
      )}
    </Field>
  );
};
