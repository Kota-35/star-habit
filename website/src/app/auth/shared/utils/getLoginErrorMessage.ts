import { match } from "ts-pattern";

const DEFAULT_FIREBASE_MESSAGE =
  "ログインに失敗しました。しばらくしてからお試しください。";

const DEFAULT_API_MESSAGE =
  "ログインに失敗しました。しばらくしてからお試しください。";

/**
 * ログイン時の Firebase Auth エラーコードをユーザー向けメッセージに変換する
 * @see https://firebase.google.com/docs/auth/admin/errors
 */
export function getFirebaseLoginErrorMessage(code: string | undefined): string {
  return match(code)
    .with(undefined, () => DEFAULT_FIREBASE_MESSAGE)
    .with(
      "auth/user-not-found",
      "auth/wrong-password",
      "auth/invalid-credential",
      () => "メールアドレスまたはパスワードが正しくありません。",
    )
    .with(
      "auth/invalid-email",
      () => "有効なメールアドレスを入力してください。",
    )
    .with(
      "auth/too-many-requests",
      () =>
        "試行回数が多すぎます。しばらく時間をおいてから再度お試しください。",
    )
    .with("auth/user-disabled", () => "このアカウントは無効化されています。")
    .with(
      "auth/network-request-failed",
      () =>
        "ネットワークエラーが発生しました。接続を確認して再度お試しください。",
    )
    .with(
      "auth/operation-not-allowed",
      () => "このログイン方法は現在利用できません。",
    )
    .with(
      "auth/email-already-in-use",
      () =>
        "このメールアドレスはすでに登録されています。（ログイン画面からお試しください）",
    )
    .otherwise(() => DEFAULT_FIREBASE_MESSAGE);
}

/**
 * サインイン API（自サーバー）の HTTP ステータスからログイン画面用のメッセージを返す
 */
export function getSigninApiErrorMessage(status: number | undefined): string {
  return match(status)
    .with(
      401,
      () =>
        "認証の有効期限が切れているか、トークンが無効です。再度ログインしてください。",
    )
    .with(
      404,
      () =>
        "このアカウントはまだ登録されていません。先にサインアップしてください。",
    )
    .with(
      500,
      () => "サーバーエラーが発生しました。しばらくしてからお試しください。",
    )
    .otherwise(() => DEFAULT_API_MESSAGE);
}
