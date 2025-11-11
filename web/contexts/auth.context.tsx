"use client";

import { Session, User } from "@supabase/supabase-js";
import { createContext, ReactNode, useContext } from "react";

interface Auth {
  user: User | null;
  session: Session | null;
}

const AuthContext = createContext<Auth | null>(null);

export const AuthProvider = ({
  children,
  user,
  session,
}: {
  children: ReactNode;
  user: User | null;
  session: Session | null;
}) => {
  return (
    <AuthContext.Provider value={{ user, session }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  let authCtx = useContext(AuthContext);

  if (!authCtx)
    throw new Error("useAuthProvider must be used within AuthContext.Provider");

  return authCtx;
};
