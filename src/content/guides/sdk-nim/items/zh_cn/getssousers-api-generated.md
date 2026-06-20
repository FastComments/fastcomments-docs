## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | int | 否 |  |

## 响应

返回: [`Option[GetSSOUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users_response.nim)

## 示例

[inline-code-attrs-start title = 'getSSOUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if response.isSome:
  let ssoUsers = response.get()
  echo ssoUsers
else:
  echo "No SSO users returned; HTTP response:", httpResponse
[inline-code-end]

---