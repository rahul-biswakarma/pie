import { createClient } from "@/lib/supabase/server";
import { type NextRequest, NextResponse } from "next/server";

export async function POST(request: NextRequest) {
  try {
    const req = await request.json();
    const roomId = req.roomId;

    if (!roomId) {
      return NextResponse.json({ error: "room_id missing" }, { status: 400 });
    }

    const supabase = await createClient();
    const user = (await supabase.auth.getUser()).data.user;

    if (!user) {
      return NextResponse.json({ error: "User not found" }, { status: 401 });
    }

    const { error } = await supabase.from("space").insert({
      slug: roomId,
      created_by: user.id,
    });

    if (error) {
      if (
        error.message.includes(
          'duplicate key value violates unique constraint "space_pkey"',
        )
      ) {
        return NextResponse.json(
          { error: "Room ID already taken" },
          { status: 409 },
        );
      }
      return NextResponse.json({ error: error.message }, { status: 500 });
    }

    return NextResponse.json({ success: true }, { status: 201 });
  } catch (e) {
    if (e instanceof Error) {
      return NextResponse.json(
        { error: `Invalid request: ${e.message}` },
        { status: 400 },
      );
    }
    return NextResponse.json({ error: "Invalid request" }, { status: 400 });
  }
}
