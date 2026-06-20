## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| commentId | string | Sí |  |
| includeEmail | bool | No |  |
| includeIP | bool | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[ModerationAPICommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_comment_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getModerationComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerationComment(commentId = "cmt-8f3b2a9d", includeEmail = false, includeIP = false, sso = "")
if response.isSome:
  let comment = response.get()
  discard comment
else:
  echo "No moderation comment returned"
[inline-code-end]

---