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
const tenantId: string = 'tenant-4f3b2c9'
const file: Blob = new Blob([new Uint8Array([137,80,78,71,13,10,26,10])], { type: 'image/png' })
const sizePreset: SizePreset = { name: 'medium', maxWidth: 1024, maxHeight: 768 }
const urlId: string = 'article-2026-01-12'
const result: UploadImageResponse = await uploadImage(tenantId, file, sizePreset, urlId)
[inline-code-end]
