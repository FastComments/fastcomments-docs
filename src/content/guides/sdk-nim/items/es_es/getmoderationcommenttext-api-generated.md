## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string = "" | No |  |

## Respuesta

Devuelve: [`Option[GetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getModerationCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getModerationCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456abc",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]