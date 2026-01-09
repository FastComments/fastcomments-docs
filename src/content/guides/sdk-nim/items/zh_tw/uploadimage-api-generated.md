## 參數

| Name | Type | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| file | string | 否 |  |
| sizePreset | SizePreset | 否 |  |
| urlId | string | 是 |  |

## 回應

回傳: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## 範例

[inline-code-attrs-start title = 'uploadImage 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.uploadImage(
  tenantId = "my-tenant-123",
  file = "assets/images/comment-avatar.jpg",
  sizePreset = SizePreset.small,
  urlId = "news/article-2025-11-22"
)
if response.isSome:
  let upload = response.get()
  echo "Uploaded image id: ", upload.id
  echo "Uploaded image url: ", upload.url
[inline-code-end]