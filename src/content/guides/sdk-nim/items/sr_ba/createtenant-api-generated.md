## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantBody | CreateTenantBody | Не |  |

## Одговор

Враћа: [`Option[CreateTenant_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример createTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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