import { createFileRoute, Outlet, useNavigate } from "@tanstack/react-router";
import SettingsHeader from "~/components/settings-header";
import { SettingHeaderProvider } from "~/providers/settings-header-provider";

export const Route = createFileRoute("/(app)/settings")({
  component: RouteComponent,
});

function RouteComponent() {
  const navigate = useNavigate();

  return (
    <div>
      {/* <SettingsHeader */}
      {/*   title="Settings" */}
      {/*   navigate={() => navigate({ to: "/profile" })} */}
      {/* /> */}
      <SettingHeaderProvider>
        <Outlet />
      </SettingHeaderProvider>
    </div>
  );
}
