## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantUserBody | CreateTenantUserBody | 否 |  |

## 响应

返回：[`Option[CreateTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user_response.nim)

## 示例

[inline-code-attrs-start title = 'createTenantUser 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenantUser(tenantId = "my-tenant-123",
  createTenantUserBody = CreateTenantUserBody(userId = "user-456",
    email = "jane.doe@example.com",
    displayName = "Jane Doe",
    roles = @["editor"],
    isAdmin = false))
if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]

---