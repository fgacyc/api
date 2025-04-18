import { S3Client, GetObjectCommand } from "@aws-sdk/client-s3";

export default defineEventHandler(async (event) => {
	const s3 = new S3Client({
		region: import.meta.env.NITRO_AWS_REGION,
		credentials: {
			accessKeyId: import.meta.env.NITRO_AWS_ACCESS_KEY_ID,
			secretAccessKey: import.meta.env.NITRO_AWS_SECRET_ACCESS_KEY,
		},
	});

	const key = getQuery(event).key as string;
	if (!key) {
		setResponseStatus(event, 400);
		return "Missing key";
	}

	const command = new GetObjectCommand({
		Bucket: import.meta.env.NITRO_S3_BUCKET,
		Key: key,
	});

	const response = await s3.send(command);

	event.node.res.setHeader(
		"Content-Type",
		response.ContentType || "application/octet-stream",
	);
	return response.Body;
});
