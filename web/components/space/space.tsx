"use client";

import { useAuth } from "@/contexts/auth.context";
import { useSocket } from "@/contexts/socket.context";
import { MeetingPage } from "@/modules/meet";
import { redirect } from "next/navigation";
import { useRef, useEffect } from "react";

export const SpacePage = ({ id }: { id: string }) => {
  const { lastJsonMessage, sendJsonMessage } = useSocket();
  const { user } = useAuth();
  const roomJoinedRef = useRef(null);

  if (!user) {
    redirect("/login");
  }

  useEffect(() => {
    sendJsonMessage({
      type: "Join",
      room: id,
      user_id: user.id,
    });
  }, [sendJsonMessage, id]);

  useEffect(() => {
    console.log(lastJsonMessage);
    if (lastJsonMessage?.type === "JoinOk") {
      sendJsonMessage({
        type: "ListParticipants",
        room: id,
      });
    }
  }, [lastJsonMessage]);

  return (
    <div>
      Space
      <div className="p-6 w-full h-[50vh]">
        <MeetingPage />
      </div>
    </div>
  );
};
