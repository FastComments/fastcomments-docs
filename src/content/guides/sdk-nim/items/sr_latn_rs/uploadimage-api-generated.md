Otpremite i promenite veličinu slike

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| file | string | Ne |  |
| options | UploadImageOptions | Ne |  |

## Odgovor

Vraća: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer uploadImage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (uploadResult, httpResponse) = client.uploadImage(
  tenantId = "my-tenant-123",
  file = "images/avatar.jpg",
  options = UploadImageOptions()
)

if uploadResult.isSome:
  let result = uploadResult.get()
  # koristite rezultat po potrebi
[inline-code-end]