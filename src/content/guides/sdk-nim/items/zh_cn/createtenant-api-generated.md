---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantBody | CreateTenantBody | 否 |  |

## 响应

返回：[`Option[CreateTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_response.nim)

## 示例

[inline-code-attrs-start title = 'createTenant 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenant(
  tenantId = "my-tenant-123",
  createTenantBody = CreateTenantBody(
    name = "My Tenant 123",
    domain = "news.example.com",
    allowAnonymous = false,
    allowedOrigins = @["https://news.example.com", "https://api.news.example.com"],
    description = "Comments for News Example"
  )
)
if response.isSome:
  let created = response.get()
  echo "Created tenant: ", created.tenantId
else:
  echo "Failed to create tenant, status: ", httpResponse.status
[inline-code-end]

---