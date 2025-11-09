import { AuthButton } from "@/components/auth/auth-button";

export default function LobbyLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800">
            <header className="border-b border-gray-700 p-4 flex justify-between items-center">
                <h1 className="text-xl font-bold text-white">Pie</h1>
                <AuthButton />
            </header>
            {children}
        </div>
    );
}
