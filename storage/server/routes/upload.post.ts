import { S3Client, PutObjectCommand } from "@aws-sdk/client-s3";
import formidable from "formidable";
import { readFile } from "node:fs/promises";
import { randomUUID } from 'node:crypto'

export default defineEventHandler(async (event) => {
    const s3 = new S3Client({
        region: import.meta.env.NITRO_AWS_REGION,
        credentials: {
            accessKeyId: import.meta.env.NITRO_AWS_ACCESS_KEY_ID,
            secretAccessKey: import.meta.env.NITRO_AWS_SECRET_ACCESS_KEY,
        },
    });

    const form = formidable({ multiples: false });
    const [fields, files] = await form.parse(event.node.req);

    const _files = files.file;
    if (_files.length === 0) return { error: "No file uploaded" };
    if (_files.length > 1) return { error: "Only uploading one file is allowed" };

    const buffer = await readFile(_files[0].filepath);

    const key = randomUUID().toString();

    await s3.send(
        new PutObjectCommand({
            Bucket: import.meta.env.NITRO_S3_BUCKET,
            Key: key,
            Body: buffer,
            ContentType: _files[0].mimetype,
        }),
    );

    return { url: `https://${import.meta.env.NITRO_DOMAIN}/api/file?key=${encodeURIComponent(key)}` };
});
