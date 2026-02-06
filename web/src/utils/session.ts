import { getCookie, setCookie } from "@tanstack/react-start/server";

const SESSION_COOKIE_NAME = "session";

export async function setSessionTokenCookie(
  token: string,
  expiresAt: Date,
): Promise<void> {
  setCookie(SESSION_COOKIE_NAME, token, {
    httpOnly: true,
    sameSite: "lax",
    secure: false, // TODO: check if its prod, if so, set to true
    path: "/",
    expires: expiresAt,
  });
}

export async function getSessionTokenCookie() {
  return getCookie(SESSION_COOKIE_NAME);
}
