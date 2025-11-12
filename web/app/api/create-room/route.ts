import { logger } from "@/lib/logger";
import { nanoid } from "nanoid";
import { type NextRequest, NextResponse } from "next/server";

export async function POST(request: NextRequest) {

	try  


    return NextResponse.json({ success: true }, { status: 200 });
  }
}

