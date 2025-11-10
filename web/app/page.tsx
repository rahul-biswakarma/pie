import Link from "next/link";
import { Button } from "@/components/ui/button";
import { createClient } from "@/lib/supabase/server";
import { MeetingPage } from "@/modules/meet";

export default async function LandingPage() {
  const supabase = await createClient();
  const {
    data: { user },
  } = await supabase.auth.getUser();

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50">
      <div className="max-w-2xl mx-auto px-4 text-center">
        <h1 className="text-5xl font-bold text-gray-900 mb-4">
          Welcome to Pie
        </h1>
        <p className="text-lg text-gray-600 mb-8">
          A new collaborative operating system.
        </p>
        <MeetingPage />
        <div className="flex gap-4 justify-center">
          <Link href={user ? "/lobby" : "/login"}>
            <Button size="lg">{user ? "Go to Lobby" : "Get Started"}</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}

