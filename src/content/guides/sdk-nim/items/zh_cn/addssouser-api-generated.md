## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createAPISSOUserData | CreateAPISSOUserData | 否 |  |

## 响应

返回: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## 示例

[inline-code-attrs-start title = 'addSSOUser 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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