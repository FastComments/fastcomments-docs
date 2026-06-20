## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di logoutPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.logoutPublic(tenantId = "my-tenant-123", sessionId = "sess-9a8b7c6d", userId = "editor-87", revokeAll = false, ipAddress = "")
if response.isSome:
  let emptyResp = response.get()
  echo "Logout successful for user: ", "editor-87"
else:
  echo "Logout failed"
[inline-code-end]