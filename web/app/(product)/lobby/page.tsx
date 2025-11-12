"use client";

import { Button } from "@/components/ui/button";
import { useSocket } from "@/contexts/socket.context";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { ReadyState } from "react-use-websocket";

export default function LobbyPage() {
  const { readyState, sendJsonMessage, lastJsonMessage } = useSocket();
  const { push } = useRouter();

  useEffect(() => {
    if (lastJsonMessage) {
      const message = lastJsonMessage as any;
      if (message.type === "CreateOK") {
        push(`/space/${message.room_id}`);
      }
    }
  }, [lastJsonMessage, push]);

  const createRoom = () => {
    sendJsonMessage({
      type: "Create",
    });
  };

  return (
    <div>
      <Button disabled={!(readyState === ReadyState.OPEN)} onClick={createRoom}>
        Create Room
      </Button>
    </div>
  );
}
