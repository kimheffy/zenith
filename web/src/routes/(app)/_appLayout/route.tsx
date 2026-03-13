import { createFileRoute, Outlet } from "@tanstack/react-router";
import Header from "~/components/header";

export const Route = createFileRoute("/(app)/_appLayout")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div>
      <Header />
      <Outlet />
    </div>
  );
}
