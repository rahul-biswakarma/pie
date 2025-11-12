import { MeetingPage } from "@/modules/meet";
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

const SpacePage = ({ id }: { id: string }) => {
	return (
		<div>
			Space
			<div className="p-6 w-full h-[50vh]">
				<MeetingPage />
			</div>
		</div>
	);
};
