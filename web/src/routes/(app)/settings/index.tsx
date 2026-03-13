import { Button } from "@base-ui/react/button";
import { createFileRoute, Link } from "@tanstack/react-router";
import { ArrowRight, Crown, Lock, Ruler, User } from "lucide-react";
import { useSettingsHeader } from "~/providers/settings-header-provider";

const SETTINGS = [
  {
    key: "account-header",
    header: "Account",
    tabs: [
      {
        key: "profile",
        icon: "user",
        text: "Profile",
        navigateTo: "/settings/profile",
      },
      {
        key: "account",
        icon: "lock",
        text: "Account",
        navigateTo: "/settings/account",
      },
      {
        key: "subscription",
        icon: "crown",
        text: "Manage Subscription",
        navigateTo: "/settings/subscriptions",
      },
    ],
  },
  {
    key: "preference-header",
    header: "Preferences",
    tabs: [
      {
        key: "units",
        icon: "ruler",
        text: "Units",
        navigateTo: "/settings/preferences",
      },
    ],
  },
];

function SettingSection({
  section,
}: {
  section: {
    header: string;
    tabs: Array<{
      key: string;
      icon: string;
      text: string;
      navigateTo: string;
    }>;
  };
}) {
  const iconMap: Record<string, typeof User> = {
    user: User,
    lock: Lock,
    crown: Crown,
    ruler: Ruler,
  } as const;

  const { handleSettingsHeader } = useSettingsHeader();

  return (
    <div>
      <h3 className="bg-gray-200 p-4">{section.header}</h3>
      {section?.tabs.map(({ key, icon, text, navigateTo }) => {
        const Icon = iconMap[icon];

        function handleNavigate() {
          handleSettingsHeader(text);
        }

        return (
          <div
            key={key}
            className="border-t-1 border-t-gray-200 last:border-b-1 last:border-b-gray-200"
          >
            <Link to={navigateTo} onClick={handleNavigate}>
              <Button className="w-full">
                <div className="flex p-4 justify-between ">
                  <div className="flex gap-2">
                    <Icon />
                    {text}
                  </div>
                  <ArrowRight />
                </div>
              </Button>
            </Link>
          </div>
        );
      })}
    </div>
  );
}

export const Route = createFileRoute("/(app)/settings/")({
  component: RouteComponent,
  loader: () => {
    return { data: SETTINGS };
  },
});

function RouteComponent() {
  const data = Route.useLoaderData();

  return (
    <div>
      {data?.data?.map(({ key, ...setting }) => (
        <SettingSection section={setting} key={key} />
      ))}
    </div>
  );
}
