## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantBody | CreateTenantBody | 아니오 |  |

## 응답

반환: [`Option[CreateTenant_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant200response.nim)

## 예제

[inline-code-attrs-start title = 'createTenant 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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