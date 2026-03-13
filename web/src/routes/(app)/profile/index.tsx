import { Button } from "@base-ui/react/button";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Settings } from "lucide-react";

export const Route = createFileRoute("/(app)/profile/")({
  component: RouteComponent,
});

function RouteComponent() {
  const navigate = useNavigate();

  return (
    <div>
      <div className="flex justify-around pt-4">
        <Button className="text-xs text-blue-700">Edit profile</Button>
        <h1>enter username</h1>
        <Button
          className="size-4"
          onClick={() => navigate({ to: "/settings" })}
        >
          <Settings />
        </Button>
      </div>
    </div>
  );
}
