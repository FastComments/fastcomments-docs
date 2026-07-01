## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | DeleteCommentOptions | No |  |

## Respuesta

Devuelve: [`Option[DeleteCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_result.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (delResult, httpResponse) = client.deleteComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  options = DeleteCommentOptions()
)

if delResult.isSome:
  let result = delResult.get()
  echo result
[inline-code-end]