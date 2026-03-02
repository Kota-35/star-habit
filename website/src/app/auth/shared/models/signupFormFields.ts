import z from "zod";

export const signupFormFieldsSchema = z.object({
  username: z.string().min(1, "usernameは必須です"),

  email: z.email("emailは必須です"),

  password: z
    .string()
    .min(8, "パスワードは8文字以上で入力してください")
    .refine(
      (val) => /\d/.test(val),
      "パスワードには数字を1文字以上含めてください",
    ),

  confirmPassword: z.string().min(1, "パスワード（確認）を入力してください"),
});

export type SignupFormFields = z.infer<typeof signupFormFieldsSchema>;
