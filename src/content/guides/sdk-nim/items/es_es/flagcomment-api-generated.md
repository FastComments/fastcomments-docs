## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de flagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(
  tenantId = "my-tenant-123",
  id = "cmt-98765",
  userId = "user-12345",
  anonUserId = ""
)

if response.isSome:
  let flagResp = response.get()
  echo "Flag response received"
else:
  echo "No flag response returned"
[inline-code-end]

---