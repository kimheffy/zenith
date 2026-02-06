import { createFileRoute } from "@tanstack/react-router";
// import { createServerFn } from "@tanstack/react-start";
// import { authMiddleware } from "~/middlewares/auth";

import { isAuthenticatedFn, assertAuthenticatedFn } from "~/fn/auth";

export const Route = createFileRoute("/(app)/workout")({
  beforeLoad: () => assertAuthenticatedFn(),
  component: RouteComponent,
});

function RouteComponent() {
  return <div>Hello "/(app)/workout"!</div>;
}
