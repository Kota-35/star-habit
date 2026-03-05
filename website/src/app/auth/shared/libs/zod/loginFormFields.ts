import z from "zod";

export const loginFormFieldsSchema = z.object({
  email: z.email("メールアドレスは必須です"),

  password: z.string().min(1, "パスワードは必須です"),
});

export type LoginFormFields = z.infer<typeof loginFormFieldsSchema>;
