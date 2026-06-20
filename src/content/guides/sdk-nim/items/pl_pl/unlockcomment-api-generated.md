---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład unLockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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