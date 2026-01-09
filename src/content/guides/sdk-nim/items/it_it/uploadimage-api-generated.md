## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| file | string | No |  |
| sizePreset | SizePreset | No |  |
| urlId | string | Sì |  |

## Risposta

Restituisce: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di uploadImage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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