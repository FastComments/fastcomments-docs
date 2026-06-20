## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| dir | int | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[GetCommentVoteUserNamesSuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names_success_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentVoteUserNames'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentVoteUserNames(tenantId = "my-tenant-123", commentId = "cmt-987654", dir = 0, sso = "")
if response.isSome:
  let success: GetCommentVoteUserNamesSuccessResponse = response.get()
  discard success
[inline-code-end]