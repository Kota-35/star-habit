import { zodResolver } from "@hookform/resolvers/zod";
import { createUserWithEmailAndPassword } from "firebase/auth";
import { useRouter } from "next/navigation";
import type React from "react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import {
  type SignupFormFields,
  signupFormFieldsSchema,
} from "@/app/auth/shared/models/signupFormFields";
import { auth } from "@/lib/firebase";
import { useSignup } from "@/server/__generated__/endpoints";

export const useSignupForm = () => {
  const router = useRouter();

  const [passwordFocus, setPasswordFocus] = useState(false);

  const toLoginFormOnClick = (() => {
    router.push("/auth/login");
  }) satisfies React.ComponentProps<"button">["onClick"];

  const {
    control,
    handleSubmit,
    register,
    setError,
    watch,
    formState: { errors },
  } = useForm<SignupFormFields>({
    resolver: zodResolver(signupFormFieldsSchema),
    defaultValues: {
      username: "",
      email: "",
      password: "",
      confirmPassword: "",
    },
  });

  const signupMutation = useSignup({
    mutation: {
      onSuccess: () => {
        console.info("ログイン！");
      },
      onError: (error) => {
        console.error(error);
      },
    },
  });

  const signupFormOnSubmit = handleSubmit(async (data: SignupFormFields) => {
    const { username, email, password } = data;

    const firebaseUser = await createUserWithEmailAndPassword(
      auth,
      email,
      password,
    )
      .then((userCredential) => {
        return userCredential.user;
      })
      .catch((error) => {
        const code =
          error && typeof error === "object" && "code" in error
            ? String(error.code)
            : undefined;
        const message =
          error && typeof error === "object" && "message" in error
            ? String(error.message)
            : undefined;

        if (code === "auth/email-already-exists") {
          router.push("/auth/login");
          throw error;
        }

        const userMessage =
          code === "auth/email-already-in-use"
            ? "このメールアドレスはすでに登録されています。"
            : (message ??
              "登録に失敗しました。しばらくしてからお試しください。");
        setError("root", { message: userMessage });
        throw error;
      });

    await signupMutation.mutateAsync({
      data: {
        username,
        email,
        firebaseUid: firebaseUser.uid,
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
    isSubmitPending: signupMutation.isPending,

    register,
    control,
    errors,

    password: watch("password"),
    passwordFocus,
    passwordFieldOnFocus,
    passwordFieldOnBlur,
  };
};
