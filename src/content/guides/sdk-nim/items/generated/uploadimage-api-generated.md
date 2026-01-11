## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| file | string | No |  |
| sizePreset | SizePreset | No |  |
| urlId | string | Yes |  |

## Response

Returns: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Example

[inline-code-attrs-start title = 'uploadImage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.uploadImage(
  tenantId = "my-tenant-123",
  file = "",
  sizePreset = default(SizePreset),
  urlId = "news/article-how-to-code"
)

if response.isSome:
  let uploadResp = response.get()
  discard uploadResp
[inline-code-end]
