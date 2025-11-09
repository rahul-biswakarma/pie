"use client";

import { useParams } from "next/navigation";

export default function RoomPage() {
  const params = useParams();
  const code = params.code as string;

  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="text-center">
        <h1 className="text-4xl font-bold">Room</h1>
        <p className="text-2xl font-mono p-4">{code}</p>
      </div>
    </div>
  );
}