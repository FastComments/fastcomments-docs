---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 响应

返回: [`Option[GetSSOUserByIdAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_id_api_response.nim)

## 示例

[inline-code-attrs-start title = 'getSSOUserById 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUserById(tenantId = "my-tenant-123", id = "user-789")
if response.isSome:
  let ssoUser: GetSSOUserByIdAPIResponse = response.get()
  echo "SSO user retrieved: ", $ssoUser
else:
  echo "No SSO user found, HTTP status: ", httpResponse.statusCode
[inline-code-end]

---