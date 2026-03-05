import { zodResolver } from "@hookform/resolvers/zod";
import type { FirebaseError } from "firebase/app";
import { useRouter } from "next/navigation";
import type React from "react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useCreateUserWithEmailAndPassword } from "@/app/auth/shared/hooks/useCreateUserWithEmailAndPassword";
import {
  type SignupFormFields,
  signupFormFieldsSchema,
} from "@/app/auth/shared/libs/zod/signupFormFields";
import { useSignup } from "@/server/__generated__/endpoints";

export const useSignupForm = () => {
  const router = useRouter();

  const [passwordFocus, setPasswordFocus] = useState(false);

  const toLoginFormOnClick = (() => {
    router.push("/auth/login");
  }) satisfies React.ComponentProps<"button">["onClick"];

  const { control, handleSubmit, register, setError, watch, formState } =
    useForm<SignupFormFields>({
      resolver: zodResolver(signupFormFieldsSchema),
      defaultValues: {
        username: "",
        email: "",
        password: "",
        confirmPassword: "",
      },
    });

  const firebaseCreateUserMutation =
    useCreateUserWithEmailAndPassword<FirebaseError>({
      onError: (error) => {
        const code = error?.code;
        const message = error?.message;

        const userMessage =
          code === "auth/email-already-in-use"
            ? "このメールアドレスはすでに登録されています。"
            : (message ??
              "登録に失敗しました。しばらくしてからお試しください。");
        setError("root", { message: userMessage });
      },
    });

  const signupMutation = useSignup({
    mutation: {
      onSuccess: ({ accessToken, refreshToken }) => {
        localStorage.setItem("accessToken", accessToken);
        localStorage.setItem("refreshToken", refreshToken);

        router.push("/home");
      },

      onError: (error) => {
        console.error("[signupMutation]", error);
        setError("root", { message: "登録に失敗しました" });
      },
    },
  });

  const signupFormOnSubmit = handleSubmit(async (data: SignupFormFields) => {
    const { username, email, password } = data;

    const userCredential = await firebaseCreateUserMutation
      .mutateAsync({ email, password })
      .catch(() => undefined);

    if (!userCredential) return;

    const idToken = await userCredential.user.getIdToken();

    await signupMutation.mutateAsync({
      data: {
        username,
        email,
        idToken,
      },
    });
  });

  const passwordFieldOnFocus = (() => {
    setPasswordFocus(true);
  }) satisfies React.ComponentProps<"input">["onFocus"];

  const passwordFieldOnBlur = (() => {
    setPasswordFocus(false);
  }) satisfies React.ComponentProps<"input">["onBlur"];

  return {
    toLoginFormOnClick,
    signupFormOnSubmit,
    isSubmitPending: formState.isSubmitting,

    register,
    control,
    errors: formState.errors,

    password: watch("password"),
    passwordFocus,
    passwordFieldOnFocus,
    passwordFieldOnBlur,
  };
};
