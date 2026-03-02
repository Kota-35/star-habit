import { createEnv } from "@t3-oss/env-nextjs";
import z from "zod";

export const env = createEnv({
  client: {
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
    NEXT_PUBLIC_SERVER_ORIGIN: z.url().describe("サーバーのURL"),
  },

  server: {
    NODE_ENV: z
      .enum(["development", "production", "test"])
      .default("development"),
    APP_ENV: z
      .enum(["development", "staging", "test", "production"])
      .default("development"),
  },

  runtimeEnv: {
    NEXT_PUBLIC_FIREBASE_API_KEY: process.env.NEXT_PUBLIC_FIREBASE_API_KEY,
    NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN:
      process.env.NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN,
    NEXT_PUBLIC_FIREBASE_PROJECT_ID:
      process.env.NEXT_PUBLIC_FIREBASE_PROJECT_ID,
    NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET:
      process.env.NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET,
    NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID:
      process.env.NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID,
    NEXT_PUBLIC_FIREBASE_APP_ID: process.env.NEXT_PUBLIC_FIREBASE_APP_ID,
    NEXT_PUBLIC_SERVER_ORIGIN: process.env.NEXT_PUBLIC_SERVER_ORIGIN,
    NODE_ENV: process.env.NODE_ENV,
    APP_ENV: process.env.APP_ENV,
  },
});

export type Env = typeof env;
