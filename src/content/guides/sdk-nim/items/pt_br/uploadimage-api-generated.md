## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| file | string | Não |  |
| sizePreset | SizePreset | Não |  |
| urlId | string | Sim |  |

## Resposta

Retorna: [`Option[UploadImageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_upload_image_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de uploadImage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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