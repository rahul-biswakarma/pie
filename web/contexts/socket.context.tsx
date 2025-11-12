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

interface Socket {
  getWebSocket: () => WebSocketLike | null;
  readyState: ReadyState;
  sendJsonMessage: (message: any, keep?: boolean) => void;
  lastJsonMessage: any;
}

const SocketContext = createContext<Socket | null>(null);

export const SocketProvider = ({ children }: { children: ReactNode }) => {
  const [socketUrl, setSocketUrl] = useState<string | null>(null);

  const { session } = useAuth();

  useEffect(() => {
    const token = session?.access_token;
    if (token) setSocketUrl(`${process.env.NEXT_PUBLIC_WS_URL}?token=${token}`);
  }, [session]);

  const { sendJsonMessage, lastJsonMessage, readyState, getWebSocket } =
    useWebSocket(
      socketUrl,
      {
        reconnectAttempts: 10,
        reconnectInterval: 3000,
        retryOnError: true,
        heartbeat: true,
      },
      !!socketUrl,
    );

  return (
    <SocketContext.Provider
      value={{ getWebSocket, readyState, sendJsonMessage, lastJsonMessage }}
    >
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
