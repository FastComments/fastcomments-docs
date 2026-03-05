Upload and resize an image

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
  file = "/var/www/uploads/headline-image.jpg",
  sizePreset = SizePreset.Medium,
  urlId = "news/article-title"
)

if response.isSome:
  let upload = response.get()
  echo "Upload response received"
else:
  echo "No upload response received"
[inline-code-end]
