import { logger } from "@/lib/logger";
import { type NextRequest, NextResponse } from "next/server";

export async function POST(request: NextRequest) {
  try {
    const entry = await request.json();

    logger.error(
      entry.message || "Client error",
      entry.error ? new Error(entry.error.message) : undefined,
      entry.context
    );

    return NextResponse.json({ success: true }, { status: 200 });
  } catch (error) {
    logger.error(
      "Failed to log error",
      error instanceof Error ? error : new Error("Unknown error")
    );
    return NextResponse.json({ success: false }, { status: 500 });
  }
}
