---
## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[LockComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_lock_comment200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di unLockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unLockComment(tenantId = "my-tenant-123", commentId = "cmt-7f9a3b2d", broadcastId = "", sso = "")
if response.isSome:
  let lockResult = response.get()
  echo "Unlock response: ", $lockResult
else:
  echo "Unlock failed, HTTP response: ", $httpResponse
[inline-code-end]

---