## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tag | string | No |  |
| tenantId | string | Sí |  |
| deleteHashTagRequest | DeleteHashTagRequest | No |  |

## Respuesta

Devuelve: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(tag = "breaking-news", tenantId = "my-tenant-123", deleteHashTagRequest = DeleteHashTagRequest())
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---