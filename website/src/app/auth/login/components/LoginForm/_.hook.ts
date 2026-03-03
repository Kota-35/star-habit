import { zodResolver } from "@hookform/resolvers/zod";
import type { AxiosError } from "axios";
import { useRouter } from "next/navigation";
import type React from "react";
import { useForm } from "react-hook-form";
import { useSignInWithEmailAndPassword } from "@/app/auth/shared/hooks/useSignInWithEmailAndPassword";
import {
  type LoginFormFields,
  loginFormFieldsSchema,
} from "@/app/auth/shared/models/loginFormFields";
import {
  getFirebaseLoginErrorMessage,
  getSigninApiErrorMessage,
} from "@/app/auth/shared/utils/getLoginErrorMessage";
import { useSignin } from "@/server/__generated__/endpoints";

export const useLoginForm = () => {
  const router = useRouter();

  const toSignupFormOnClick = (() => {
    router.push("/auth/signup");
  }) satisfies React.ComponentProps<"button">["onClick"];

  const { handleSubmit, setError, register, control, formState } =
    useForm<LoginFormFields>({
      resolver: zodResolver(loginFormFieldsSchema),
      defaultValues: {
        email: "",
        password: "",
      },
    });

  const firebaseSignInMutation = useSignInWithEmailAndPassword({
    onError: (error) => {
      const userMessage = getFirebaseLoginErrorMessage(error?.code);
      setError("root", { message: userMessage });
    },
  });

  const signinMutation = useSignin({
    mutation: {
      onSuccess: ({ accessToken, refreshToken }) => {
        localStorage.setItem("accessToken", accessToken);
        localStorage.setItem("refreshToken", refreshToken);

        router.push("/home");
      },

      onError: (error) => {
        const status = (error as AxiosError)?.response?.status;
        const userMessage = getSigninApiErrorMessage(status);
        setError("root", { message: userMessage });
      },
    },
  });

  const loginFormOnSubmit = handleSubmit(async (data: LoginFormFields) => {
    const { email, password } = data;

    const userCredential = await firebaseSignInMutation.mutateAsync({
      email,
      password,
    });

    const idToken = await userCredential.user.getIdToken();

    await signinMutation.mutateAsync({
      data: {
        idToken,
      },
    });
  });

  return {
    toSignupFormOnClick,

    errors: formState.errors,
    isSubmitPending: formState.isSubmitting,
    control,
    register,
    loginFormOnSubmit,
  };
};
