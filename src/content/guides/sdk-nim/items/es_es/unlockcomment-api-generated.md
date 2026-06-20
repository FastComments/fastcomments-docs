## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unLockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let commentId = "cmt-987654321"
let (response, httpResponse) = client.unLockComment(
  tenantId = tenantId,
  commentId = commentId,
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let apiResp = response.get()
  echo "Unlocked comment ", commentId, " for tenant ", tenantId
else:
  echo "Unlock failed, HTTP status: ", $httpResponse.status
[inline-code-end]

---