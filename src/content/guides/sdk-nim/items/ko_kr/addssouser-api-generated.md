## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createAPISSOUserData | CreateAPISSOUserData | 아니요 |  |

## 응답

반환: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## 예제

[inline-code-attrs-start title = 'addSSOUser 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addSSOUser(
  tenantId = "my-tenant-123",
  createAPISSOUserData = CreateAPISSOUserData(
    id = "sso-456",
    email = "alice.johnson@newsorg.com",
    name = "Alice Johnson",
    roles = @["editor", "contributor"],
    isActive = true,
    isAdmin = false
  )
)
if response.isSome:
  let apiResp = response.get()
  discard apiResp
else:
  discard httpResponse
[inline-code-end]

---