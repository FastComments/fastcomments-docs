---
Subir y redimensionar una imagen

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Yes |  |
| file | string | No |  |
| options | UploadImageOptions | No |  |

## Respuesta

Devuelve: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de uploadImage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (uploadResult, httpResponse) = client.uploadImage(
  tenantId = "my-tenant-123",
  file = "images/avatar.jpg",
  options = UploadImageOptions()
)

if uploadResult.isSome:
  let result = uploadResult.get()
  # usar el resultado según sea necesario
[inline-code-end]

---