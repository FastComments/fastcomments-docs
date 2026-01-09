## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| file | string | Nej |  |
| sizePreset | SizePreset | Nej |  |
| urlId | string | Ja |  |

## Svar

Returnerer: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Eksempel

[inline-code-attrs-start title = 'uploadImage Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---