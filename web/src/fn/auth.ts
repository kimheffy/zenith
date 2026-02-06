import { redirect } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";
import { validateReq } from "~/utils/auth";
import { getCookie } from "./cookie";

export const isAuthenticatedFn = createServerFn().handler(async () => {
  const foundCookie = await getCookie();
  return !!foundCookie;
});

export const assertAuthenticatedFn = createServerFn().handler(async () => {
  const foundCookie = await getCookie();

  if (!foundCookie) {
    throw redirect({ to: "/login" });
  }

  const validated = validateReq(foundCookie);

  return validated;
});
