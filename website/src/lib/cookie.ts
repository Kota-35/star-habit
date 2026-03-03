export function setCookie(
  name: string,
  value: string,
  options: { path?: string; maxAge?: number } = {},
): void {
  const { path = "/", maxAge } = options;
  const parts = [`${name}=${value}`, `path=${path}`];
  if (maxAge !== undefined) parts.push(`max-age=${maxAge}`);
  // biome-ignore lint/suspicious/noDocumentCookie: minimal cookie setter for sidebar state; Cookie Store API has limited support
  document.cookie = parts.join("; ");
}
