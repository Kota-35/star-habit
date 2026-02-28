import { z } from "zod";

/**
 * 環境変数のスキーマ
 * @see https://zenn.dev/kiwichan101kg/articles/7ba33ab64414b2
 */
export const envSchema = z.object({
  NEXT_PUBLIC_FIREBASE_API_KEY: z
    .string()
    .trim()
    .min(1, "required")
    .describe("Firebase API Key"),
  NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN: z
    .string()
    .trim()
    .min(1, "required")
    .describe("Firebase Auth ドメイン"),
  NEXT_PUBLIC_FIREBASE_PROJECT_ID: z
    .string()
    .trim()
    .min(1, "required")
    .describe("Firebase プロジェクト ID"),
  NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET: z
    .string()
    .trim()
    .min(1, "required")
    .describe("Firebase Storage バケット"),
  NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID: z
    .string()
    .trim()
    .min(1, "required")
    .describe("Firebase Cloud Messaging の Sender ID"),
  NEXT_PUBLIC_FIREBASE_APP_ID: z
    .string()
    .trim()
    .min(1, "required")
    .describe("Firebase アプリ ID"),
});

export type Env = z.infer<typeof envSchema>;
