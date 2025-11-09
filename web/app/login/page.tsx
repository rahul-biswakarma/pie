import { SignInForm } from "@/components/auth/sign-in-form";
import Link from "next/link";
import { Button } from "@/components/ui/button";

export default function LoginPage() {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 p-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <Link href="/">
            <h1 className="text-4xl font-bold text-gray-900 mb-2 cursor-pointer hover:text-blue-600 transition-colors">
              Pie
            </h1>
          </Link>
          <p className="text-gray-600">Sign in to start or join meetings</p>
        </div>

        <SignInForm />

        <div className="mt-6 text-center">
          <Link href="/">
            <Button variant="ghost">Back to Home</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}
