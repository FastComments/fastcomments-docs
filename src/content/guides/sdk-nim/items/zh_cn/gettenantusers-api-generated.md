## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 响应

返回: [`Option[GetTenantUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users_response.nim)

## 示例

[inline-code-attrs-start title = 'getTenantUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUsers(tenantId = "my-tenant-123", skip = 0.0)

if response.isSome:
  let tenantUsers = response.get()
  echo "Retrieved tenant users"
else:
  echo "No tenant users returned"
[inline-code-end]

---