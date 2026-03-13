import { useRouter } from "@tanstack/react-router";
import { createContext, useContext, useState } from "react";
import SettingsHeader from "~/components/settings-header";

const SettingHeaderContext = createContext<{
  settingsHeader: string;
  handleSettingsHeader: (header: string) => void;
} | null>(null);

function SettingHeaderProvider({ children }: { children: React.ReactNode }) {
  const router = useRouter();

  const [settingsHeader, setSettingsHeader] = useState("Settings");

  function handleSettingsHeader(header: string) {
    setSettingsHeader(header);
  }

  const value = { settingsHeader, handleSettingsHeader };

  return (
    <SettingHeaderContext.Provider value={value}>
      <SettingsHeader
        title={settingsHeader}
        navigate={() => router.history.go(-1)}
      />
      {children}
    </SettingHeaderContext.Provider>
  );
}

function useSettingsHeader() {
  const context = useContext(SettingHeaderContext);
  if (!context) {
    throw new Error(
      "useSettingsHeader must be used within a SettingsHeaderProvider",
    );
  }

  return context;
}

export { SettingHeaderProvider, useSettingsHeader };
