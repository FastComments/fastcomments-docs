## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| blockFromCommentParams | BlockFromCommentParams | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de blockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-98765",
  blockFromCommentParams = BlockFromCommentParams(),
  userId = "user-456",
  anonUserId = ""
)
if response.isSome:
  let blocked = response.get()
  echo "Block confirmed for tenant:", " my-tenant-123"
[inline-code-end]

---