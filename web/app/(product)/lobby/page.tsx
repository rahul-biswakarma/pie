"use client";

import { Button } from "@/components/ui/button";
import { redirect, useRouter } from "next/navigation";

export default function LobbyPage() {
  const createRoom = async () => {
    await fetch("/api/create-room", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ roomId: "rahul" }),
    }).then((res) => {
      if (res.ok) {
        redirect("/space/rahul");
      }
    });
  };

  return (
    <div>
      <Button onClick={createRoom}>Create Room</Button>
    </div>
  );
}
