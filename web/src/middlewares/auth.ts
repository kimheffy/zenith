import { redirect } from "@tanstack/react-router";
import { createMiddleware } from "@tanstack/react-start";
import { getCookie } from "~/fn/cookie";

export const authMiddleware = createMiddleware({ type: "function" }).client(
  async ({ next }) => {
    const cookieFound = await getCookie();
    if (!cookieFound) {
      throw redirect({ to: "/login" });
    }

    return next({
      headers: {
        Authorization: `Bearer ${cookieFound}`,
      },
    });
  },
);
