import { Button } from "@base-ui/react/button";
import { ArrowLeft } from "lucide-react";

function SettingsHeader({
  title,
  navigate,
}: {
  title: string;
  navigate: () => void;
}) {
  return (
    <div className="flex px-4 justify-center items-center py-4">
      <Button onClick={navigate}>
        <ArrowLeft />
      </Button>
      <h1 className="m-auto">{title}</h1>
    </div>
  );
}

export default SettingsHeader;
