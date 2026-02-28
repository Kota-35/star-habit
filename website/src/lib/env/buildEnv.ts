import { envSchema } from "./envSchema";

const parsedEnv = envSchema.safeParse(process.env);

if (!parsedEnv.success) {
  console.error(
    new Error(
      `❌ 無効な環境変数です: ${JSON.stringify(parsedEnv.error.issues, null, 2)}`,
    ),
  );

  throw new Error("無効な環境変数です");
}

export const env = parsedEnv.data;
