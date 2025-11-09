"use client";

import { User } from "@supabase/supabase-js";
import { createContext, ReactNode, useContext } from "react";

interface Auth {
    user: User | null;
}

const AuthContext = createContext<Auth | null>(null);

export const AuthProvider = ({
    children,
    user,
}: {
    children: ReactNode;
    user: User | null;
}) => {
    return (
        <AuthContext.Provider value={{ user }}>{children}</AuthContext.Provider>
    );
};

export const useAuthProvider = () => {
    let authCtx = useContext(AuthContext);

    if (!authCtx)
        throw new Error(
            "useAuthProvider must be used within AuthContext.Provider",
        );

    return authCtx;
};
