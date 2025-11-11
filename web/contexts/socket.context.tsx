"use client";

import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { WebSocketLike } from "react-use-websocket/dist/lib/types";
import { useAuth } from "./auth.context";
import { logger } from "@/lib/logger";

interface Socket {
  getWebSocket: () => WebSocketLike | null;
  readyState: ReadyState;
}

const SocketContext = createContext<Socket | null>(null);

export const SocketProvider = ({ children }: { children: ReactNode }) => {
  const [socketUrl, setSocketUrl] = useState<string | null>(null);

  const { session } = useAuth();

  useEffect(() => {
    const token = session?.access_token;
    if (token)
      setSocketUrl(
        `${process.env.SOCKET_URL ?? "ws://127.0.0.1:3001/socket"}?token=${token}`,
      );
  }, [session]);

  const { sendMessage, lastMessage, readyState, getWebSocket } = useWebSocket(
    socketUrl,
    {
      reconnectAttempts: 10,
      reconnectInterval: 3000,
      retryOnError: true,
      heartbeat: true,
      onOpen: () => {
        logger.info("WS connection stablised");
      },
    },
    !!socketUrl,
  );

  return (
    <SocketContext.Provider value={{ getWebSocket, readyState }}>
      {children}
    </SocketContext.Provider>
  );
};

export const useSocket = () => {
  let socketCtx = useContext(SocketContext);

  if (!socketCtx) {
    throw new Error("useSocket should be called withtin the SocketProvider");
  }

  return socketCtx;
};
