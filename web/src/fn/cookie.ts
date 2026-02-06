import { redirect } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";
import { validateReq } from "~/utils/auth";
import { getSessionTokenCookie, setSessionTokenCookie } from "~/utils/session";

export const getCookie = createServerFn().handler(async () => {
  return getSessionTokenCookie();
});

export const setCookieAndRedirect = createServerFn()
  .inputValidator((data: { access_token: string }) => data)
  .handler(async ({ data }) => {
    const { exp } = validateReq(data.access_token);
    await setSessionTokenCookie(data.access_token, new Date(exp * 1000));
    throw redirect({ to: "/workout" });
  });
