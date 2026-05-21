Upload and resize an image

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| file | Blob | Yes |  |
| sizePreset | SizePreset | No |  |
| urlId | string | No |  |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UploadImageResponse.ts)

## Example

[inline-code-attrs-start title = 'uploadImage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-acme-42';
const file: Blob = new Blob([Uint8Array.from([137, 80, 78, 71, 13, 10])], { type: 'image/png' });
const sizePreset: SizePreset = { name: 'hero-lg', maxWidth: 1920, maxHeight: 1080 };
const urlId: string = 'homepage-hero-2026';
const result: UploadImageResponse = await uploadImage(tenantId, file, sizePreset, urlId);
[inline-code-end]
