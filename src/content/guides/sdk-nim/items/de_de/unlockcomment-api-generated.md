## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'unLockComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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