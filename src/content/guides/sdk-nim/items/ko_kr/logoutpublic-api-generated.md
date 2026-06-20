## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'logoutPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.logoutPublic(tenantId = "my-tenant-123", sessionId = "sess-9a8b7c6d", userId = "editor-87", revokeAll = false, ipAddress = "")
if response.isSome:
  let emptyResp = response.get()
  echo "Logout successful for user: ", "editor-87"
else:
  echo "Logout failed"
[inline-code-end]

---