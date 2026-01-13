## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantBody | CreateTenantBody | 否 |  |

## 回應

回傳: [`Option[CreateTenant_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant200response.nim)

## 範例

[inline-code-attrs-start title = 'createTenant 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenant(tenantId = "my-tenant-123", createTenantBody = CreateTenantBody(
  name: "My Tenant 123",
  domain: "mytenant.example.com",
  plan: "pro",
  isActive: true,
  allowedOrigins: @["https://www.example.com", "https://admin.example.com"]
))
if response.isSome:
  let tenantInfo = response.get()
  discard tenantInfo
[inline-code-end]

---