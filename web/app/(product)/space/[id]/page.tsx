import { SpacePage } from "@/components/space/space";
import { redirect } from "next/navigation";

export default async function SpacePageWrapper({
  params,
}: {
  params: Promise<{ [key: string]: string | string[] }>;
}) {
  const { id } = await params;
  if (!id) {
    redirect("/lobby");
  }

  return <SpacePage id={id as string} />;
}
