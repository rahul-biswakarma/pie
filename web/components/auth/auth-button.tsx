"use client";

import { Button } from "@/components/ui/button";
import { ErrorModule } from "@/lib/errors";
import { logger } from "@/lib/logger";
import { createClient } from "@/lib/supabase/client";
import { useRouter } from "next/navigation";
import { useState } from "react";

export function AuthButton() {
  const router = useRouter();
  const [isLoading, setIsLoading] = useState(false);
  const supabase = createClient();

  const handleSignOut = async () => {
    setIsLoading(true);
    try {
      await supabase.auth.signOut();
      router.refresh();
    } catch (error) {
      logger.error(
        "Error signing out",
        error instanceof Error ? error : new Error("Unknown error"),
        {},
        ErrorModule.Shared
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Button onClick={handleSignOut} disabled={isLoading} variant="outline">
      {isLoading ? "Signing out..." : "Sign Out"}
    </Button>
  );
}
