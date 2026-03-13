// import { Link } from "@tanstack/react-router";
import { Button } from "@base-ui/react/button";
import { useNavigate } from "@tanstack/react-router";
import { Plus, User } from "lucide-react";

export default function Header() {
  const navigate = useNavigate();

  return (
    <header className="p-4 flex items-center bg-white text-black backdrop-blur-xs justify-between">
      <Button
        className="p-1 transition-colors border rounded-full border-gray-500"
        aria-label="User icon"
        onClick={() => navigate({ to: "/profile" })}
      >
        <User size={16} />
      </Button>
      <h1>zenith</h1>
      <Button
        className="p-1 transition-colors border rounded-full border-gray-500"
        aria-label="Add icon"
      >
        <Plus size={16} />
      </Button>
    </header>
  );
}
