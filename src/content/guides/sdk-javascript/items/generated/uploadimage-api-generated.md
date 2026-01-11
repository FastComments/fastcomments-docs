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
const tenantId: string = "fc-tenant-84b9f3c2";
const imageData: Uint8Array = new Uint8Array([255,216,255,224,0,16,74,70,73,70]);
const file: Blob = new Blob([imageData], { type: "image/jpeg" });
const sizePreset: SizePreset = { name: "medium", maxWidth: 1024, maxHeight: 768 };
const urlId: string = "products/blue-widget-2025/hero-image";
const uploadResult: UploadImageResponse = await uploadImage(tenantId, file, sizePreset, urlId);
const uploadResultMinimal: UploadImageResponse = await uploadImage(tenantId, file);
[inline-code-end]
