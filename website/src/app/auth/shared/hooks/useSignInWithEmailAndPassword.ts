import {
  type UseMutationOptions,
  type UseMutationResult,
  useMutation,
} from "@tanstack/react-query";
import type { FirebaseError } from "firebase/app";
import { signInWithEmailAndPassword, type UserCredential } from "firebase/auth";
import { auth } from "@/lib/firebase";

interface SignInWithEmailAndPasswordVariables {
  email: string;
  password: string;
}

export const useSignInWithEmailAndPassword = <TContext = unknown>(
  options?: UseMutationOptions<
    UserCredential,
    FirebaseError,
    SignInWithEmailAndPasswordVariables,
    TContext
  >,
): UseMutationResult<
  UserCredential,
  FirebaseError,
  SignInWithEmailAndPasswordVariables,
  TContext
> => {
  return useMutation({
    mutationFn: ({ email, password }: SignInWithEmailAndPasswordVariables) =>
      signInWithEmailAndPassword(auth, email, password),
    ...options,
  });
};
