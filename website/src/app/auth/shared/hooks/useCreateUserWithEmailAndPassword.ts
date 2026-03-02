import type {
  UseMutationOptions,
  UseMutationResult,
} from "@tanstack/react-query";
import { useMutation } from "@tanstack/react-query";
import type { FirebaseError } from "firebase/app";
import {
  createUserWithEmailAndPassword,
  type UserCredential,
} from "firebase/auth";
import { auth } from "@/lib/firebase";

interface CreateUserWithEmailAndPasswordVariables {
  email: string;
  password: string;
}

export const useCreateUserWithEmailAndPassword = <TContext = unknown>(
  options?: UseMutationOptions<
    UserCredential,
    FirebaseError,
    CreateUserWithEmailAndPasswordVariables,
    TContext
  >,
): UseMutationResult<
  UserCredential,
  FirebaseError,
  CreateUserWithEmailAndPasswordVariables,
  TContext
> => {
  return useMutation({
    mutationFn: ({
      email,
      password,
    }: CreateUserWithEmailAndPasswordVariables) =>
      createUserWithEmailAndPassword(auth, email, password),
    ...options,
  });
};
